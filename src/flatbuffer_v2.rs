use std::collections::HashMap;

use flatbuffers::{FlatBufferBuilder, ForwardsUOffset, UnionWIPOffset, Vector, WIPOffset};
use serde_json::Value;

use crate::stylesheet_v2_generated::styles_v2;

// JSON -> flatbuffer 转换器
//
// 体积优化：
// 1. 字符串统一进 StyleSheet.string_pool 池，引用处只存 4 字节下标
//    （declaration 字符串值 -> PoolString、Selector.string_id、KeyValueString）
// 2. 相同内容的 value table（PoolString/Integer/Double/Boolean/各种数组）以及
//    DeclarationTuple 只创建一次，重复引用同一 offset（flatbuffers 支持共享子对象）
// 3. 相同内容的 declaration/selector 向量也只存一份，Style 里重复引用同一向量
type DeclarationVector<'a> =
  WIPOffset<Vector<'a, ForwardsUOffset<styles_v2::DeclarationTuple<'a>>>>;
type SelectorVector<'a> = WIPOffset<Vector<'a, styles_v2::Selector>>;

struct FlatbufferConverter<'a> {
  builder: FlatBufferBuilder<'a>,
  // 字符串池：内容 -> 下标，offsets 按下标顺序存放
  string_pool: HashMap<String, u32>,
  string_pool_offsets: Vec<WIPOffset<&'a str>>,
  // value table 去重缓存
  pool_string_cache: HashMap<u32, WIPOffset<styles_v2::PoolString<'a>>>,
  integer_cache: HashMap<i64, WIPOffset<styles_v2::Integer<'a>>>,
  double_cache: HashMap<u64, WIPOffset<styles_v2::Double<'a>>>,
  boolean_cache: [Option<WIPOffset<styles_v2::Boolean<'a>>>; 2],
  string_array_cache: HashMap<Vec<u32>, WIPOffset<styles_v2::StringArray<'a>>>,
  integer_array_cache: HashMap<Vec<i64>, WIPOffset<styles_v2::IntegerArray<'a>>>,
  double_array_cache: HashMap<Vec<u64>, WIPOffset<styles_v2::DoubleArray<'a>>>,
  // DeclarationTuple 去重缓存：(property_id, value_type, value_offset, flag)
  declaration_cache: HashMap<(u8, u8, u32, u8), WIPOffset<styles_v2::DeclarationTuple<'a>>>,
  // 向量级去重缓存：declaration offsets / selector 内容
  declaration_vector_cache: HashMap<Vec<u32>, DeclarationVector<'a>>,
  selector_vector_cache: HashMap<Vec<(u32, u8, bool)>, SelectorVector<'a>>,
}

impl<'a> FlatbufferConverter<'a> {
  fn new() -> Self {
    Self {
      builder: FlatBufferBuilder::new(),
      string_pool: HashMap::new(),
      string_pool_offsets: Vec::new(),
      pool_string_cache: HashMap::new(),
      integer_cache: HashMap::new(),
      double_cache: HashMap::new(),
      boolean_cache: [None, None],
      string_array_cache: HashMap::new(),
      integer_array_cache: HashMap::new(),
      double_array_cache: HashMap::new(),
      declaration_cache: HashMap::new(),
      declaration_vector_cache: HashMap::new(),
      selector_vector_cache: HashMap::new(),
    }
  }

  // 字符串入池，返回 string_pool 下标
  fn intern_string(&mut self, s: &str) -> u32 {
    if let Some(&id) = self.string_pool.get(s) {
      return id;
    }
    let offset = self.builder.create_string(s);
    let id = self.string_pool_offsets.len() as u32;
    self.string_pool_offsets.push(offset);
    self.string_pool.insert(s.to_string(), id);
    id
  }

  fn process_value(&mut self, value: &Value) -> (styles_v2::Value, WIPOffset<UnionWIPOffset>) {
    match value {
      Value::String(s) => {
        let id = self.intern_string(s);
        let pool_string = self.pool_string_cache.entry(id).or_insert_with(|| {
          styles_v2::PoolString::create(&mut self.builder, &styles_v2::PoolStringArgs { id })
        });
        (styles_v2::Value::PoolString, pool_string.as_union_value())
      }
      Value::Number(n) => {
        if n.is_f64() {
          let bits = n.as_f64().unwrap().to_bits();
          let double = self.double_cache.entry(bits).or_insert_with(|| {
            styles_v2::Double::create(
              &mut self.builder,
              &styles_v2::DoubleArgs {
                value: n.as_f64().unwrap(),
              },
            )
          });
          (styles_v2::Value::Double, double.as_union_value())
        } else {
          let int_value = n.as_i64().unwrap();
          let integer = self.integer_cache.entry(int_value).or_insert_with(|| {
            styles_v2::Integer::create(&mut self.builder, &styles_v2::IntegerArgs { value: int_value })
          });
          (styles_v2::Value::Integer, integer.as_union_value())
        }
      }
      Value::Bool(b) => {
        let index = *b as usize;
        if self.boolean_cache[index].is_none() {
          let boolean =
            styles_v2::Boolean::create(&mut self.builder, &styles_v2::BooleanArgs { value: *b });
          self.boolean_cache[index] = Some(boolean);
        }
        (
          styles_v2::Value::Boolean,
          self.boolean_cache[index].unwrap().as_union_value(),
        )
      }
      Value::Array(arr) => {
        if arr.len() > 0 {
          match &arr[0] {
            Value::String(_) => self.create_string_array_value(arr),
            Value::Number(_) => {
              let is_integer = arr.iter().all(|n| n.is_i64());
              if is_integer {
                let values: Vec<i64> = arr.iter().map(|n| n.as_i64().unwrap()).collect();
                let array = self.integer_array(&values);
                (styles_v2::Value::IntegerArray, array.as_union_value())
              } else {
                let values: Vec<f64> = arr.iter().map(|n| n.as_f64().unwrap()).collect();
                let array = self.double_array(&values);
                (styles_v2::Value::DoubleArray, array.as_union_value())
              }
            }
            Value::Object(_) => {
              let key_values: Vec<_> = arr
                .iter()
                .map(|obj| {
                  let (value_type, value) = self.process_value(obj);
                  styles_v2::KeyValue::create(
                    &mut self.builder,
                    &styles_v2::KeyValueArgs {
                      key: None,
                      value_type,
                      value: Some(value),
                    },
                  )
                })
                .collect();
              let fields_offset = self.builder.create_vector(&key_values);
              let array = styles_v2::ObjectArray::create(
                &mut self.builder,
                &styles_v2::ObjectArrayArgs {
                  values: Some(fields_offset),
                },
              );
              (styles_v2::Value::ObjectArray, array.as_union_value())
            }
            Value::Array(_) => {
              let is_integer = arr.iter().all(|n| n.is_i64());
              if is_integer {
                self.create_integer_array_array_value(arr)
              } else {
                self.create_double_array_array_value(arr)
              }
            }
            _ => {
              println!("{:?}", value);
              panic!("Invalid array type")
            }
          }
        } else {
          // 处理空数组，返回一个空的整数数组
          let array = self.integer_array(&[]);
          (styles_v2::Value::IntegerArray, array.as_union_value())
        }
      }
      Value::Object(obj) => self.create_object_value(obj),
      _ => {
        println!("{:?}", value);
        panic!("Invalid value type")
      }
    }
  }

  // IntegerArray table 去重创建
  fn integer_array(&mut self, values: &[i64]) -> WIPOffset<styles_v2::IntegerArray<'a>> {
    if let Some(&cached) = self.integer_array_cache.get(values) {
      return cached;
    }
    let vector = self.builder.create_vector(values);
    let array = styles_v2::IntegerArray::create(
      &mut self.builder,
      &styles_v2::IntegerArrayArgs {
        values: Some(vector),
      },
    );
    self.integer_array_cache.insert(values.to_vec(), array);
    array
  }

  // DoubleArray table 去重创建
  fn double_array(&mut self, values: &[f64]) -> WIPOffset<styles_v2::DoubleArray<'a>> {
    let bits: Vec<u64> = values.iter().map(|v| v.to_bits()).collect();
    if let Some(&cached) = self.double_array_cache.get(&bits) {
      return cached;
    }
    let vector = self.builder.create_vector(values);
    let array = styles_v2::DoubleArray::create(
      &mut self.builder,
      &styles_v2::DoubleArrayArgs {
        values: Some(vector),
      },
    );
    self.double_array_cache.insert(bits, array);
    array
  }

  fn create_string_array_value(
    &mut self,
    arr: &Vec<Value>,
  ) -> (styles_v2::Value, WIPOffset<UnionWIPOffset>) {
    let ids: Vec<u32> = arr
      .iter()
      .map(|s| self.intern_string(s.as_str().unwrap()))
      .collect();
    if let Some(&cached) = self.string_array_cache.get(&ids) {
      return (styles_v2::Value::StringArray, cached.as_union_value());
    }
    let values = self.builder.create_vector(&ids);
    let array = styles_v2::StringArray::create(
      &mut self.builder,
      &styles_v2::StringArrayArgs {
        values: Some(values),
      },
    );
    self.string_array_cache.insert(ids, array);
    (styles_v2::Value::StringArray, array.as_union_value())
  }

  fn create_integer_array_array_value(
    &mut self,
    arr: &Vec<Value>,
  ) -> (styles_v2::Value, WIPOffset<UnionWIPOffset>) {
    let integer_values: Vec<_> = arr
      .iter()
      .map(|n| {
        let values: Vec<i64> = n
          .as_array()
          .unwrap()
          .iter()
          .map(|n| n.as_i64().unwrap())
          .collect();
        self.integer_array(&values)
      })
      .collect();
    let integers = self.builder.create_vector(&integer_values);
    let array = styles_v2::IntegereArrayArray::create(
      &mut self.builder,
      &styles_v2::IntegereArrayArrayArgs {
        values: Some(integers),
      },
    );
    (styles_v2::Value::IntegereArrayArray, array.as_union_value())
  }

  fn create_double_array_array_value(
    &mut self,
    arr: &Vec<Value>,
  ) -> (styles_v2::Value, WIPOffset<UnionWIPOffset>) {
    let double_values: Vec<_> = arr
      .iter()
      .map(|n| {
        let values: Vec<f64> = n
          .as_array()
          .unwrap()
          .iter()
          .map(|n| n.as_f64().unwrap())
          .collect();
        self.double_array(&values)
      })
      .collect();
    let doubles = self.builder.create_vector(&double_values);
    let array = styles_v2::DoubleArrayArray::create(
      &mut self.builder,
      &styles_v2::DoubleArrayArrayArgs {
        values: Some(doubles),
      },
    );
    (styles_v2::Value::DoubleArrayArray, array.as_union_value())
  }

  fn create_object_value(
    &mut self,
    obj: &serde_json::Map<String, Value>,
  ) -> (styles_v2::Value, WIPOffset<UnionWIPOffset>) {
    let key_values: Vec<_> = obj
      .iter()
      .map(|(key, value)| {
        let key_offset = self.builder.create_shared_string(key);
        let (value_type, value_offset) = self.process_value(value);

        styles_v2::KeyValue::create(
          &mut self.builder,
          &styles_v2::KeyValueArgs {
            key: Some(key_offset),
            value_type,
            value: Some(value_offset),
          },
        )
      })
      .collect();

    let fields_offset = self.builder.create_vector(&key_values);
    let object = styles_v2::Object::create(
      &mut self.builder,
      &styles_v2::ObjectArgs {
        fields: Some(fields_offset),
      },
    );

    (styles_v2::Value::Object, object.as_union_value())
  }

  // declaration 向量去重创建
  fn declaration_vector(
    &mut self,
    decls: &[WIPOffset<styles_v2::DeclarationTuple<'a>>],
  ) -> DeclarationVector<'a> {
    let key: Vec<u32> = decls.iter().map(|d| d.value()).collect();
    if let Some(&cached) = self.declaration_vector_cache.get(&key) {
      return cached;
    }
    let vector = self.builder.create_vector(decls);
    self.declaration_vector_cache.insert(key, vector);
    vector
  }

  // selector 向量去重创建
  fn selector_vector(&mut self, selectors: &[styles_v2::Selector]) -> SelectorVector<'a> {
    let key: Vec<(u32, u8, bool)> = selectors
      .iter()
      .map(|s| (s.string_id(), s.integer_value(), s.is_string()))
      .collect();
    if let Some(&cached) = self.selector_vector_cache.get(&key) {
      return cached;
    }
    let vector = self.builder.create_vector(selectors);
    self.selector_vector_cache.insert(key, vector);
    vector
  }

  // DeclarationTuple 去重创建
  fn create_declaration(&mut self, decl: &Value) -> WIPOffset<styles_v2::DeclarationTuple<'a>> {
    let decl_array = decl.as_array().unwrap();
    let property_id = decl_array[0].as_u64().unwrap() as u8;

    let (value_type, value) = self.process_value(&decl_array[1]);

    let mut property_flag = 0;
    // 判断下标2是否存在
    if decl_array.len() == 3 {
      property_flag = decl_array[2].as_u64().unwrap() as u8;
    }

    let cache_key = (property_id, value_type.0, value.value(), property_flag);
    if let Some(&cached) = self.declaration_cache.get(&cache_key) {
      return cached;
    }
    let declaration = styles_v2::DeclarationTuple::create(
      &mut self.builder,
      &styles_v2::DeclarationTupleArgs {
        property_id,
        value_type,
        value: Some(value),
        flag: property_flag,
      },
    );
    self.declaration_cache.insert(cache_key, declaration);
    declaration
  }

  fn create_condition(&mut self, cond: &serde_json::Value) -> WIPOffset<styles_v2::Condition<'a>> {
    let cond_array = cond.as_array().unwrap();
    let cond_type = cond_array[0].as_u64().unwrap() as u8;
    let condition_value = match cond_type {
      0 => {
        // PrimitiveCondition
        let params = cond_array[1].as_array().unwrap();
        let feature = params[0].as_u64().unwrap() as u8;
        let operator = params[1].as_u64().unwrap() as u8;
        // 处理value值
        let (value_type, value) = self.process_value(&params[2]);

        // 创建PrimitiveCondition
        let primitive = styles_v2::PrimitiveCondition::create(
          &mut self.builder,
          &styles_v2::PrimitiveConditionArgs {
            feature,
            operator,
            value_type,
            value: Some(value),
          },
        );
        styles_v2::Condition::create(
          &mut self.builder,
          &styles_v2::ConditionArgs {
            type_: cond_type,
            value_type: styles_v2::ConditionValue::PrimitiveCondition,
            value: Some(primitive.as_union_value()),
          },
        )
      }
      _ => {
        let compound_conditions: Vec<_> = cond_array[1]
          .as_array()
          .unwrap()
          .iter()
          .map(|cond| self.create_condition(cond))
          .collect();

        let vec_offset = self.builder.create_vector(&compound_conditions);
        let compound = styles_v2::CompoundCondition::create(
          &mut self.builder,
          &styles_v2::CompoundConditionArgs {
            conditions: Some(vec_offset),
          },
        );

        styles_v2::Condition::create(
          &mut self.builder,
          &styles_v2::ConditionArgs {
            type_: cond_type,
            value_type: styles_v2::ConditionValue::CompoundCondition,
            value: Some(compound.as_union_value()),
          },
        )
      }
    };
    condition_value
  }
}

pub fn convert_json_to_flatbuffer(json_str: &str) -> Result<Vec<u8>, serde_json::Error> {
  let json: Value = serde_json::from_str(json_str)?;
  let mut converter = FlatbufferConverter::new();
  let fonts: Vec<WIPOffset<styles_v2::Font>> = json["fonts"]
    .as_array()
    .unwrap()
    .iter()
    .map(|f| {
      let font_family = f["fontFamily"].as_str().unwrap_or("");
      let src = f["src"].as_str().unwrap_or("");

      let font_family = converter.builder.create_shared_string(font_family);
      let src = converter.builder.create_shared_string(src);
      styles_v2::Font::create(
        &mut converter.builder,
        &styles_v2::FontArgs {
          font_family: Some(font_family),
          src: Some(src),
        },
      )
    })
    .collect();
  let fonts = converter.builder.create_vector(&fonts);

  // 处理keyframes
  let keyframes: Vec<WIPOffset<styles_v2::KeyframeAnimation>> = json["keyframes"]
    .as_array()
    .unwrap()
    .iter()
    .map(|k| {
      let name = k["name"].as_str().unwrap_or("");
      let name_offset = converter.builder.create_shared_string(name);
      let media = k["media"].as_u64().unwrap_or(0) as u8;

      // 处理关键帧点
      let keyframe_points: Vec<WIPOffset<styles_v2::KeyframeAnimationPoint>> = k["keyframe"]
        .as_array()
        .unwrap()
        .iter()
        .map(|point| {
          let percentage = point["percent"].as_f64().unwrap_or(0.0) as f32;

          // 处理每个关键帧点的样式声明
          let declarations: Vec<WIPOffset<styles_v2::DeclarationTuple>> = point["event"]
            .as_array()
            .unwrap()
            .iter()
            .map(|decl| converter.create_declaration(decl))
            .collect();

          let declarations = converter.declaration_vector(&declarations);

          // 创建关键帧点
          styles_v2::KeyframeAnimationPoint::create(
            &mut converter.builder,
            &styles_v2::KeyframeAnimationPointArgs {
              percentage,
              declarations: Some(declarations),
            },
          )
        })
        .collect();

      let keyframe_points = converter.builder.create_vector(&keyframe_points);

      // 创建关键帧动画
      styles_v2::KeyframeAnimation::create(
        &mut converter.builder,
        &styles_v2::KeyframeAnimationArgs {
          name: Some(name_offset),
          media,
          keyframe_points: Some(keyframe_points),
        },
      )
    })
    .collect();

  let keyframes = converter.builder.create_vector(&keyframes);

  let medias: Vec<WIPOffset<styles_v2::Media>> = json["medias"]
    .as_array()
    .unwrap()
    .iter()
    .map(|m| {
      let conditions: Vec<_> = m["conditions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|cond_array| converter.create_condition(cond_array))
        .collect();
      let conditions_vector = converter.builder.create_vector(&conditions);
      styles_v2::Media::create(
        &mut converter.builder,
        &styles_v2::MediaArgs {
          id: m["id"].as_u64().unwrap() as u8,
          conditions: Some(conditions_vector),
        },
      )
    })
    .collect();
  let medias = converter.builder.create_vector(&medias);

  let styles: Vec<WIPOffset<styles_v2::Style>> = json["styles"]
    .as_array()
    .unwrap()
    .iter()
    .map(|style| {
      let selector: Vec<styles_v2::Selector> = style["selector"]
        .as_array()
        .unwrap()
        .iter()
        .map(|sel| match sel {
          Value::String(s) => {
            let string_id = converter.intern_string(s);
            styles_v2::Selector::new(string_id, 0, true)
          }
          Value::Number(n) => styles_v2::Selector::new(0, n.as_u64().unwrap() as u8, false),
          _ => panic!("Invalid selector type"),
        })
        .collect();
      let selector = converter.selector_vector(&selector);

      let declarations: Vec<WIPOffset<styles_v2::DeclarationTuple>> = style["declarations"]
        .as_array()
        .unwrap()
        .iter()
        .map(|decl| converter.create_declaration(decl))
        .collect();
      let declarations = converter.declaration_vector(&declarations);
      let pseudo_key: Vec<styles_v2::PseudoKey> = style["pseudo_key"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .map(|key| {
          if key.is_i64() {
            styles_v2::PseudoKey::new(key.as_i64().unwrap() as i8, false, true)
          } else {
            styles_v2::PseudoKey::new(0, key.as_bool().unwrap(), false)
          }
        })
        .collect();
      let pseudo_key_build = converter.builder.create_vector(&pseudo_key);
      let pseudo_val = if let Some(pseudo_val) = style["pseudo_val"].as_str() {
        Some(converter.builder.create_shared_string(pseudo_val))
      } else {
        None
      };

      // variables: Object {"--color": String("red")}，key/value 均为 string_pool 下标
      let variables: Vec<styles_v2::KeyValueString> = style["variables"]
        .as_object()
        .unwrap_or(&serde_json::Map::new())
        .iter()
        .map(|(key, value)| {
          let key_id = converter.intern_string(key);
          let value_id = converter.intern_string(value.as_str().unwrap());
          styles_v2::KeyValueString::new(key_id, value_id)
        })
        .collect();

      let variables_build = converter.builder.create_vector(&variables);

      styles_v2::Style::create(
        &mut converter.builder,
        &styles_v2::StyleArgs {
          declarations: Some(declarations),
          media: style["media"].as_u64().unwrap() as u8,
          pseudo: style["pseudo"].as_u64().unwrap_or(0) as u8,
          selector: Some(selector),
          pseudo_key: if pseudo_key.len() > 0 {
            Some(pseudo_key_build)
          } else {
            None
          },
          pseudo_val,
          variables: if variables.len() > 0 {
            Some(variables_build)
          } else {
            None
          },
        },
      )
    })
    .collect();
  let styles = converter.builder.create_vector(&styles);
  let design_width = json["design_width"].as_u64().unwrap_or(0) as u16;
  let allow_inherit = json["allow_inherit"].as_bool().unwrap_or(false);

  // 字符串池统一挂到 StyleSheet 上
  let string_pool_offsets = converter.string_pool_offsets.clone();
  let string_pool = converter.builder.create_vector(&string_pool_offsets);

  let stylesheet = styles_v2::StyleSheet::create(
    &mut converter.builder,
    &styles_v2::StyleSheetArgs {
      fonts: Some(fonts),
      keyframes: Some(keyframes),
      medias: Some(medias),
      styles: Some(styles),
      string_pool: Some(string_pool),
      design_width,
      allow_inherit,
    },
  );

  converter.builder.finish(stylesheet, None);
  Ok(converter.builder.finished_data().to_vec())
}
