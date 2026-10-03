// 属性注册表：CSSPropertyType 枚举、string→类型 映射、match 派发 的唯一事实源。
// 用法：for_each_property_entry! { macro_name } 把整张表传给下级宏展开。
//
// 字段：变体名, 数值 id, 是否复合属性, StyleValueType 派发(包装变体(负载类型) 参数源), CSS 名(kebab), camelCase 名, 别名 camel...
// 参数源枚举：
//   - pname  : 派发时传 property_name（vendor 前缀摘除 + 首字母小写，保留原 camelCase 名）
//   - id     : 派发时传底层 PropertyId 的原始 camelCase 字符串 id
//   - special: 属性需逐属性手写派发（fontFamily、content、zIndex、gridTemplate/…）
//   - skip   : 属性具有 id 但从不在 match 派发中出现（如 BackgroundPositionX/Y、Animation*）
// 注意：数值 id 是线上产物格式，禁止改动既有值；新增属性只能在表尾追加（>=129）；
//       复合属性的 camel 名仅用于 var() 变量路径的 id 反查（与旧 string_to_css_property_type 行为一致）。
#[macro_export]
macro_rules! for_each_property_entry {
  ($m:ident) => {
    $m! {
      // 布局 / flex
      AlignContent,           1,   simple,  FlexAlign(FlexAlign) pname,             "align-content",            "alignContent";
      JustifyContent,         2,   simple,  FlexAlign(FlexAlign) pname,             "justify-content",          "justifyContent";
      AlignItems,             3,   simple,  AlignItems(ItemAlign) pname,            "align-items",              "alignItems";
      AlignSelf,              4,   simple,  AlignItems(ItemAlign) pname,            "align-self",               "alignSelf";
      FlexBasis,              5,   simple,  FlexBasis(FlexBasis) pname,             "flex-basis",               "flexBasis";
      FlexDirection,          6,   simple,  FlexDirection(FlexDirection) pname,     "flex-direction",           "flexDirection";
      FlexGrow,               7,   simple,  NumberProperty(NumberProperty) pname,   "flex-grow",                "flexGrow";
      FlexShrink,             8,   simple,  NumberProperty(NumberProperty) pname,   "flex-shrink",              "flexShrink";
      FlexWrap,               9,   simple,  FlexWrap(FlexWrap) pname,               "flex-wrap",                "flexWrap";
      AspectRatio,            10,  simple,  AspectRatio(AspectRatio) pname,         "aspect-ratio",             "aspectRatio";
      Display,                11,  simple,  Display(Display) pname,                 "display",                  "display";
      ColumnGap,              12,  simple,  Gap(Gap) id,                            "column-gap",               "columnGap";
      RowGap,                 13,  simple,  Gap(Gap) id,                            "row-gap",                  "rowGap";
      // 间距 / 定位偏移
      MarginLeft,             14,  simple,  LengthValueProperty(LengthValueProperty) id, "margin-left",              "marginLeft";
      MarginRight,            15,  simple,  LengthValueProperty(LengthValueProperty) id, "margin-right",             "marginRight";
      MarginTop,              16,  simple,  LengthValueProperty(LengthValueProperty) id, "margin-top",               "marginTop";
      MarginBottom,           17,  simple,  LengthValueProperty(LengthValueProperty) id, "margin-bottom",            "marginBottom";
      PaddingLeft,            18,  simple,  LengthValueProperty(LengthValueProperty) id, "padding-left",             "paddingLeft";
      PaddingRight,           19,  simple,  LengthValueProperty(LengthValueProperty) id, "padding-right",            "paddingRight";
      PaddingTop,             20,  simple,  LengthValueProperty(LengthValueProperty) id, "padding-top",              "paddingTop";
      PaddingBottom,          21,  simple,  LengthValueProperty(LengthValueProperty) id, "padding-bottom",           "paddingBottom";
      // 尺寸
      Width,                  22,  simple,  SizeProperty(SizeProperty) id,          "width",                    "width";
      MinWidth,               23,  simple,  SizeProperty(SizeProperty) id,          "min-width",                "minWidth";
      MaxWidth,               24,  simple,  MaxSizeProperty(MaxSizeProperty) id,    "max-width",                "maxWidth";
      Height,                 25,  simple,  SizeProperty(SizeProperty) id,          "height",                   "height";
      MinHeight,              26,  simple,  SizeProperty(SizeProperty) id,          "min-height",               "minHeight";
      MaxHeight,              27,  simple,  MaxSizeProperty(MaxSizeProperty) id,    "max-height",               "maxHeight";
      Overflow,               28,  simple,  Overflow(Overflow) id,                  "overflow",                 "overflow";
      // 文字
      FontSize,               29,  simple,  FontSize(FontSize) id,                  "font-size",                "fontSize";
      FontStyle,              30,  simple,  FontStyle(FontStyle) id,                "font-style",               "fontStyle";
      FontFamily,             31,  simple,  Special(Special) special,               "font-family",              "fontFamily";
      FontWeight,             32,  simple,  FontWeight(FontWeight) id,              "font-weight",              "fontWeight";
      LineHeight,             33,  simple,  LineHeight(LineHeight) id,              "line-height",              "lineHeight";
      LetterSpacing,          34,  simple,  LetterSpacing(LetterSpacing) id,        "letter-spacing",           "letterSpacing";
      VerticalAlign,          35,  simple,  VerticalAlign(VerticalAlign) id,        "vertical-align",           "verticalAlign";
      TextAlign,              36,  simple,  TextAlign(TextAlign) id,                "text-align",               "textAlign";
      TextDecoration,         37,  simple,  TextDecoration(TextDecoration) id,      "text-decoration",          "textDecoration";
      TextShadow,             38,  simple,  TextShadow(TextShadow) id,              "text-shadow",              "textShadow";
      TextOverflow,           39,  simple,  TextOverflow(TextOverflow) id,          "text-overflow",            "textOverflow";
      TextTransform,          40,  simple,  TextTransform(TextTransform) id,        "text-transform",           "textTransform";
      Color,                  41,  simple,  ColorProperty(ColorProperty) id,        "color",                    "color";
      // 背景
      BackgroundColor,        42,  simple,  ColorProperty(ColorProperty) id,        "background-color",         "backgroundColor";
      BackgroundImage,        43,  simple,  BackgroundImage(BackgroundImage) id,    "background-image",         "backgroundImage";
      BackgroundPosition,     44,  simple,  BackgroundPosition(BackgroundPosition) id, "background-position",      "backgroundPosition";
      BackgroundSize,         45,  simple,  BackgroundSize(BackgroundSize) id,      "background-size",          "backgroundSize";
      BackgroundRepeat,       46,  simple,  BackgroundRepeat(BackgroundRepeat) id,  "background-repeat",        "backgroundRepeat";
      // 边框（单边）
      BorderTopColor,         47,  simple,  BorderColor(BorderColor) id,            "border-top-color",         "borderTopColor";
      BorderRightColor,       48,  simple,  BorderColor(BorderColor) id,            "border-right-color",       "borderRightColor";
      BorderBottomColor,      49,  simple,  BorderColor(BorderColor) id,            "border-bottom-color",      "borderBottomColor";
      BorderLeftColor,        50,  simple,  BorderColor(BorderColor) id,            "border-left-color",        "borderLeftColor";
      BorderTopStyle,         51,  simple,  BorderStyle(BorderStyle) id,            "border-top-style",         "borderTopStyle";
      BorderRightStyle,       52,  simple,  BorderStyle(BorderStyle) id,            "border-right-style",       "borderRightStyle";
      BorderBottomStyle,      53,  simple,  BorderStyle(BorderStyle) id,            "border-bottom-style",      "borderBottomStyle";
      BorderLeftStyle,        54,  simple,  BorderStyle(BorderStyle) id,            "border-left-style",        "borderLeftStyle";
      BorderTopWidth,         55,  simple,  BorderWidth(BorderWidth) id,            "border-top-width",         "borderTopWidth";
      BorderRightWidth,       56,  simple,  BorderWidth(BorderWidth) id,            "border-right-width",       "borderRightWidth";
      BorderBottomWidth,      57,  simple,  BorderWidth(BorderWidth) id,            "border-bottom-width",      "borderBottomWidth";
      BorderLeftWidth,        58,  simple,  BorderWidth(BorderWidth) id,            "border-left-width",        "borderLeftWidth";
      BorderTopLeftRadius,    59,  simple,  BorderRadius(BorderRadius) id,          "border-top-left-radius",   "borderTopLeftRadius";
      BorderTopRightRadius,   60,  simple,  BorderRadius(BorderRadius) id,          "border-top-right-radius",  "borderTopRightRadius";
      BorderBottomLeftRadius, 61,  simple,  BorderRadius(BorderRadius) id,          "border-bottom-left-radius","borderBottomLeftRadius";
      BorderBottomRightRadius,62,  simple,  BorderRadius(BorderRadius) id,          "border-bottom-right-radius","borderBottomRightRadius";
      // 阴影 / 定位 / 其他
      BoxShadow,              63,  simple,  BoxShadow(BoxShadow) id,                "box-shadow",               "boxShadow";
      ZIndex,                 64,  simple,  Special(Special) special,               "z-index",                  "zIndex";
      Position,               65,  simple,  Position(Position) id,                  "position",                 "position";
      Top,                    66,  simple,  LengthValueProperty(LengthValueProperty) id, "top",                      "top";
      Right,                  67,  simple,  LengthValueProperty(LengthValueProperty) id, "right",                    "right";
      Bottom,                 68,  simple,  LengthValueProperty(LengthValueProperty) id, "bottom",                   "bottom";
      Left,                   69,  simple,  LengthValueProperty(LengthValueProperty) id, "left",                     "left";
      Visibility,             70,  simple,  Visibility(Visibility) id,              "visibility",               "visibility";
      Opacity,                71,  simple,  Opacity(Opacity) id,                    "opacity",                  "opacity";
      Transform,              72,  simple,  Transform(Transform) id,                "transform",                "transform";
      TransformOrigin,        73,  simple,  TransformOrigin(TransformOrigin) id,    "transform-origin",         "transformOrigin";
      // 动画
      AnimationKeyFrames,     74,  simple,  Skip(Skip) skip,                        "animation-keyframes",      "animationKeyFrames";
      AnimationDuration,      75,  simple,  AnimationMulti(AnimationMulti) id,      "animation-duration",       "animationDuration";
      AnimationTimingFunction,76,  simple,  AnimationMulti(AnimationMulti) id,      "animation-timing-function","animationTimingFunction";
      AnimationDelay,         77,  simple,  AnimationMulti(AnimationMulti) id,      "animation-delay",          "animationDelay";
      AnimationIterationCount,78,  simple,  AnimationMulti(AnimationMulti) id,      "animation-iteration-count","animationIterationCount";
      Content,                79,  simple,  Special(Special) special,               "content",                  "content";
      WordBreak,              80,  simple,  WordBreak(WordBreak) id,                "word-break",               "wordBreak";
      WebkitLineClamp,        81,  simple,  Special(Special) special,               "-webkit-line-clamp",       "lineClamp",              "webkitLineClamp";
      AnimationFillMode,      82,  simple,  AnimationMulti(AnimationMulti) id,      "animation-fill-mode",      "animationFillMode";
      BackgroundPositionX,    83,  simple,  Skip(Skip) skip,                        "background-position-x",    "backgroundPositionX";
      BackgroundPositionY,    84,  simple,  Skip(Skip) skip,                        "background-position-y",    "backgroundPositionY";
      // 过渡
      Transition,             85,  compound,Transition(Transition) id,              "transition",               "transition";
      TransitionProperty,     86,  simple,  Transition(Transition) id,              "transition-property",      "transitionProperty";
      TransitionDuration,     87,  simple,  Transition(Transition) id,              "transition-duration",      "transitionDuration";
      TransitionTimingFunction,88, simple,  Transition(Transition) id,              "transition-timing-function","transitionTimingFunction";
      TransitionDelay,        89,  simple,  Transition(Transition) id,              "transition-delay",         "transitionDelay";
      WhiteSpace,             90,  simple,  WhiteSpace(WhiteSpace) id,              "white-space",              "whiteSpace";
      TextDecorationLine,     91,  simple,  TextDecoration(TextDecoration) id,      "text-decoration-line",     "textDecorationLine";
      TextDecorationThickness,92,  simple,  TextDecoration(TextDecoration) id,      "text-decoration-thickness","textDecorationThickness";
      TextDecorationStyle,    93,  simple,  TextDecoration(TextDecoration) id,      "text-decoration-style",    "textDecorationStyle";
      TextDecorationColor,    94,  simple,  TextDecoration(TextDecoration) id,      "text-decoration-color",    "textDecorationColor";
      AnimationName,          95,  simple,  AnimationMulti(AnimationMulti) id,      "animation-name",           "animationName";
      BorderWidth,            96,  compound,BorderWidth(BorderWidth) id,            "border-width",             "borderWidth";
      BorderColor,            97,  compound,BorderColor(BorderColor) id,            "border-color",             "borderColor";
      Margin,                 98,  compound,MarginPadding(MarginPadding) id,        "margin",                   "margin";
      Padding,                99,  compound,MarginPadding(MarginPadding) id,        "padding",                  "padding";
      BorderRadius,           100, compound,BorderRadius(BorderRadius) id,          "border-radius",            "borderRadius";
      BoxOrient,              101, simple,  BoxOrient(BoxOrient) id,                "box-orient",               "boxOrient";
      PointerEvents,          102, simple,  PointerEvents(PointerEvents) id,        "pointer-events",           "pointerEvents", "PointerEvents";
      Background,             103, compound,Background(Background) id,              "background",               "background";
      Flex,                   104, compound,Flex(Flex) id,                          "flex",                     "flex";
      Border,                 105, compound,Border(Border) id,                      "border",                   "border";
      BorderStyle,            106, compound,BorderStyle(BorderStyle) id,            "border-style",             "borderStyle";
      Gap,                    107, compound,Gap(Gap) id,                            "gap",                      "gap";
      AnimationDirection,     108, simple,  Skip(Skip) skip,                        "animation-direction",      "animationDirection";
      AnimationPlayState,     109, simple,  Skip(Skip) skip,                        "animation-play-state",     "animationPlayState";
      Animation,              110, compound,AnimationMulti(AnimationMulti) id,      "animation",                "animation";
      Filter,                 111, simple,  Special(Special) special,               "filter",                   "filter";
      BorderTop,              112, compound,Border(Border) id,                      "border-top",               "borderTop";
      BorderRight,            113, compound,Border(Border) id,                      "border-right",             "borderRight";
      BorderBottom,           114, compound,Border(Border) id,                      "border-bottom",            "borderBottom";
      BorderLeft,             115, compound,Border(Border) id,                      "border-left",              "borderLeft";
      TextUnderlineOffset,    116, simple,  Special(Special) special,               "text-underline-offset",    "textUnderlineOffset";
      BackdropFilter,         117, simple,  Special(Special) special,               "backdrop-filter",          "backdropFilter";
      // 网格布局（跨端基础集）
      GridTemplateColumns,    118, simple,  Special(Special) special,               "grid-template-columns",    "gridTemplateColumns";
      GridTemplateRows,       119, simple,  Special(Special) special,               "grid-template-rows",       "gridTemplateRows";
      GridRowStart,           120, simple,  GridPlacement(GridPlacement) pname,     "grid-row-start",           "gridRowStart";
      GridRowEnd,             121, simple,  GridPlacement(GridPlacement) pname,     "grid-row-end",             "gridRowEnd";
      GridColumnStart,        122, simple,  GridPlacement(GridPlacement) pname,     "grid-column-start",        "gridColumnStart";
      GridColumnEnd,          123, simple,  GridPlacement(GridPlacement) pname,     "grid-column-end",          "gridColumnEnd";
      GridRow,                124, compound,GridPlacement(GridPlacement) pname,     "grid-row",                 "gridRow";
      GridColumn,             125, compound,GridPlacement(GridPlacement) pname,     "grid-column",              "gridColumn";
      GridArea,               126, compound,GridPlacement(GridPlacement) pname,     "grid-area",                "gridArea";
      JustifyItems,           127, simple,  AlignItems(ItemAlign) pname,            "justify-items",            "justifyItems";
      JustifySelf,            128, simple,  AlignItems(ItemAlign) pname,            "justify-self",             "justifySelf";
      // 网格布局（完整集补充：隐式轨道 + 自动放置流）
      GridAutoRows,           129, simple,  Special(Special) special,               "grid-auto-rows",           "gridAutoRows";
      GridAutoColumns,        130, simple,  Special(Special) special,               "grid-auto-columns",        "gridAutoColumns";
      GridAutoFlow,           131, simple,  GridAutoFlow(GridAutoFlow) pname,       "grid-auto-flow",           "gridAutoFlow";
      // place-* 正常声明展开为长属性；保留 ID 用于 var()/env() 字符串回退。
      PlaceItems,             132, compound,Special(Special) special,               "place-items",              "placeItems";
      PlaceSelf,              133, compound,Special(Special) special,               "place-self",               "placeSelf";
      PlaceContent,           134, compound,Special(Special) special,               "place-content",            "placeContent";
    }
  };
}

// 派生占位类型：special / skip 不使用具体负载类型
// NOTE：Special 和 Skip 需要是合法 ident 才能通过 macro 模式匹配；
// 实际派发逻辑对 special rows 由 parse_style_properties.rs 中保留的手写臂处理，
// 对 skip rows 不生成任何派发臂。
#[derive(Debug, Clone)]
pub struct Special;

#[derive(Debug, Clone)]
pub struct Skip;
