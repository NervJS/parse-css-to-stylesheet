
#[repr(u32)] 
#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub enum Alignment {
  TopStart, // 顶部起始端。
  Top, // 顶部横向居中。
  TopEnd, // 顶部尾端。
  Start, // 起始端纵向居中。
  Center, // 横向和纵向居中。
  End, // 尾端纵向居中。
  BottomStart, // 底部起始端。
  Bottom, // 底部横向居中。
  BottomEnd, // 底部尾端。
}

#[repr(u32)] 
#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub enum ImageRepeat {
  X, // 只在水平轴上重复绘制图片。
  Y, // 只在竖直轴上重复绘制图片。
  XY, // 在两个轴上重复绘制图片。
  NoRepeat, // 不重复绘制图片。
}


#[repr(u32)] 
#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub enum ImageSize {
  Cover, // 默认值，保持宽高比进行缩小或者放大，使得图片两边都大于或等于显示边界。
  Contain, // 保持宽高比进行缩小或者放大，使得图片完全显示在显示边界内。
  Auto, // 保持原图的比例不变。
}

#[repr(u32)] 
#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub enum BorderStyle {
  Dotted, // 显示为一系列圆点，圆点半径为borderWidth的一半。
  Dashed, // 显示为一系列短的方形虚线。
  Solid, // 显示为一条实线。
}

#[repr(u32)] 
#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub enum FlexAlign {
  Start, //元素在主轴方向首端对齐，第一个元素与行首对齐，同时后续的元素与前一个对齐。
  Center, // 元素在主轴方向中心对齐，第一个元素与行首的距离与最后一个元素与行尾距离相同。
  End, // 元素在主轴方向尾部对齐，最后一个元素与行尾对齐，其他元素与后一个对齐。
  SpaceBetween, // Flex主轴方向均匀分配弹性元素，相邻元素之间距离相同。第一个元素与行首对齐，最后一个元素与行尾对齐。
  SpaceAround, // Flex主轴方向均匀分配弹性元素，相邻元素之间距离相同。第一个元素到行首的距离和最后一个元素到行尾的距离是相邻元素之间距离的一半。
  SpaceEvenly, // Flex主轴方向均匀分配弹性元素，相邻元素之间的距离、第一个元素与行首的间距、最后一个元素到行尾的间距都完全一样。
}

#[repr(u32)] 
#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub enum FlexDirection {
  Row, // 主轴与行方向一致作为布局模式。
  RowReverse, // 与Row方向相反方向进行布局。
  Column, // 主轴与列方向一致作为布局模式。
  ColumnReverse, // 与Column相反方向进行布局。
}

#[repr(u32)] 
#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub enum FlexWrap {
  NoWrap, // Flex容器的元素单行/列布局，子项不允许超出容器。
  Wrap, // Flex容器的元素多行/列排布，子项允许超出容器。
  WrapReverse, // Flex容器的元素反向多行/列排布，子项允许超出容器。
}

#[repr(u32)] 
#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub enum FontStyle {
  Normal, // 标准的字体样式。
  Italic, // 斜体的字体样式。
}

#[repr(u32)] 
#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub enum FontWeight {
  Lighter, // 字体较细。
  Normal, // 字体粗细正常。
  Regular, // 字体粗细正常。
  Medium, // 字体粗细适中。
  Bold, // 字体较粗。
  Bolder, // 字体非常粗。
}

#[repr(u32)] 
#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub enum ItemAlign {
  Auto, // 使用Flex容器中默认配置。
  Start, // 元素在Flex容器中，交叉轴方向首部对齐。
  Center, // 元素在Flex容器中，交叉轴方向居中对齐。
  End, // 元素在Flex容器中，交叉轴方向底部对齐。
  Stretch, // 元素在Flex容器中，交叉轴方向拉伸填充。容器为Flex且设置Wrap为FlexWrap.Wrap或FlexWrap.WrapReverse时，元素拉伸到与当前行/列交叉轴长度最长的元素尺寸。其余情况下，无论元素尺寸是否设置时，均拉伸到容器尺寸。
  Baseline, // 元素在Flex容器中，交叉轴方向文本基线对齐。
}


#[repr(u32)] 
#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub enum TextAlign {
  Start, // 水平对齐首部。
  Center, // 水平居中对齐。
  End, // 水平对齐尾部。
  JUSTIFY, // 双端对齐。
}


#[repr(u32)] 
#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub enum TextDecorationLine {
  Underline, // 文字下划线修饰。
  LineThrough, // 穿过文本的修饰线。
  Overline, // 文字上划线修饰。
  None, // 不使用文本装饰线。
}

#[repr(u32)] 
#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub enum TextOverflow {
  None, // 文本超长时按最大行截断显示。
  Clip, // 文本超长时按最大行截断显示。
  Ellipsis, // 文本超长时显示不下的文本用省略号代替。
  MARQUEE, // 文本超长时以跑马灯的方式展示。
}

#[repr(u32)] 
#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub enum TextCase {
  Normal, // 保持文本原有大小写。
  LowerCase, // 文本采用全小写。
  UpperCase, // 文本采用全大写。
}

#[repr(u32)] 
#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub enum Overflow {
  Visible,
  Hidden, 
  Scroll,
}


#[repr(u32)] 
#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub enum Display {
  None,
  Block, 
  Flex,
}


#[repr(u32)] 
#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub enum Position {
  Static,
  Relative,
  Absolute, 
  Fixed,
  Sticky
}


#[repr(u32)] 
#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub enum Visibility {
  Visible,
  Hidden
}

