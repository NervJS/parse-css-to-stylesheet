// 属性注册表：CSSPropertyType 枚举、string→类型 映射、match 派发 的唯一事实源。
// 用法：for_each_property_entry! { macro_name } 把整张表传给下级宏展开。
//
// 字段：变体名, 数值 id, 是否复合属性, CSS 名(kebab), camelCase 名, 别名 camel...
// 注意：数值 id 是线上产物格式，禁止改动既有值；复合属性的 camel 名仅用于
// CSS 变量(var())路径的 id 反查（与旧 string_to_css_property_type 行为一致）。
#[macro_export]
macro_rules! for_each_property_entry {
  ($m:ident) => {
    $m! {
      // 布局 / flex
      AlignContent,           1,   simple,  "align-content",            "alignContent";
      JustifyContent,         2,   simple,  "justify-content",          "justifyContent";
      AlignItems,             3,   simple,  "align-items",              "alignItems";
      AlignSelf,              4,   simple,  "align-self",               "alignSelf";
      FlexBasis,              5,   simple,  "flex-basis",               "flexBasis";
      FlexDirection,          6,   simple,  "flex-direction",           "flexDirection";
      FlexGrow,               7,   simple,  "flex-grow",                "flexGrow";
      FlexShrink,             8,   simple,  "flex-shrink",              "flexShrink";
      FlexWrap,               9,   simple,  "flex-wrap",                "flexWrap";
      AspectRatio,            10,  simple,  "aspect-ratio",             "aspectRatio";
      Display,                11,  simple,  "display",                  "display";
      ColumnGap,              12,  simple,  "column-gap",               "columnGap";
      RowGap,                 13,  simple,  "row-gap",                  "rowGap";
      // 间距 / 定位偏移
      MarginLeft,             14,  simple,  "margin-left",              "marginLeft";
      MarginRight,            15,  simple,  "margin-right",             "marginRight";
      MarginTop,              16,  simple,  "margin-top",               "marginTop";
      MarginBottom,           17,  simple,  "margin-bottom",            "marginBottom";
      PaddingLeft,            18,  simple,  "padding-left",             "paddingLeft";
      PaddingRight,           19,  simple,  "padding-right",            "paddingRight";
      PaddingTop,             20,  simple,  "padding-top",              "paddingTop";
      PaddingBottom,          21,  simple,  "padding-bottom",           "paddingBottom";
      // 尺寸
      Width,                  22,  simple,  "width",                    "width";
      MinWidth,               23,  simple,  "min-width",                "minWidth";
      MaxWidth,               24,  simple,  "max-width",                "maxWidth";
      Height,                 25,  simple,  "height",                   "height";
      MinHeight,              26,  simple,  "min-height",               "minHeight";
      MaxHeight,              27,  simple,  "max-height",               "maxHeight";
      Overflow,               28,  simple,  "overflow",                 "overflow";
      // 文字
      FontSize,               29,  simple,  "font-size",                "fontSize";
      FontStyle,              30,  simple,  "font-style",               "fontStyle";
      FontFamily,             31,  simple,  "font-family",              "fontFamily";
      FontWeight,             32,  simple,  "font-weight",              "fontWeight";
      LineHeight,             33,  simple,  "line-height",              "lineHeight";
      LetterSpacing,          34,  simple,  "letter-spacing",           "letterSpacing";
      VerticalAlign,          35,  simple,  "vertical-align",           "verticalAlign";
      TextAlign,              36,  simple,  "text-align",               "textAlign";
      TextDecoration,         37,  simple,  "text-decoration",          "textDecoration";
      TextShadow,             38,  simple,  "text-shadow",              "textShadow";
      TextOverflow,           39,  simple,  "text-overflow",            "textOverflow";
      TextTransform,          40,  simple,  "text-transform",           "textTransform";
      Color,                  41,  simple,  "color",                    "color";
      // 背景
      BackgroundColor,        42,  simple,  "background-color",         "backgroundColor";
      BackgroundImage,        43,  simple,  "background-image",         "backgroundImage";
      BackgroundPosition,     44,  simple,  "background-position",      "backgroundPosition";
      BackgroundSize,         45,  simple,  "background-size",          "backgroundSize";
      BackgroundRepeat,       46,  simple,  "background-repeat",        "backgroundRepeat";
      // 边框（单边）
      BorderTopColor,         47,  simple,  "border-top-color",         "borderTopColor";
      BorderRightColor,       48,  simple,  "border-right-color",       "borderRightColor";
      BorderBottomColor,      49,  simple,  "border-bottom-color",      "borderBottomColor";
      BorderLeftColor,        50,  simple,  "border-left-color",        "borderLeftColor";
      BorderTopStyle,         51,  simple,  "border-top-style",         "borderTopStyle";
      BorderRightStyle,       52,  simple,  "border-right-style",       "borderRightStyle";
      BorderBottomStyle,      53,  simple,  "border-bottom-style",      "borderBottomStyle";
      BorderLeftStyle,        54,  simple,  "border-left-style",        "borderLeftStyle";
      BorderTopWidth,         55,  simple,  "border-top-width",         "borderTopWidth";
      BorderRightWidth,       56,  simple,  "border-right-width",       "borderRightWidth";
      BorderBottomWidth,      57,  simple,  "border-bottom-width",      "borderBottomWidth";
      BorderLeftWidth,        58,  simple,  "border-left-width",        "borderLeftWidth";
      BorderTopLeftRadius,    59,  simple,  "border-top-left-radius",   "borderTopLeftRadius";
      BorderTopRightRadius,   60,  simple,  "border-top-right-radius",  "borderTopRightRadius";
      BorderBottomLeftRadius, 61,  simple,  "border-bottom-left-radius","borderBottomLeftRadius";
      BorderBottomRightRadius,62,  simple,  "border-bottom-right-radius","borderBottomRightRadius";
      // 阴影 / 定位 / 其他
      BoxShadow,              63,  simple,  "box-shadow",               "boxShadow";
      ZIndex,                 64,  simple,  "z-index",                  "zIndex";
      Position,               65,  simple,  "position",                 "position";
      Top,                    66,  simple,  "top",                      "top";
      Right,                  67,  simple,  "right",                    "right";
      Bottom,                 68,  simple,  "bottom",                   "bottom";
      Left,                   69,  simple,  "left",                     "left";
      Visibility,             70,  simple,  "visibility",               "visibility";
      Opacity,                71,  simple,  "opacity",                  "opacity";
      Transform,              72,  simple,  "transform",                "transform";
      TransformOrigin,        73,  simple,  "transform-origin",         "transformOrigin";
      // 动画
      AnimationKeyFrames,     74,  simple,  "animation-keyframes",      "animationKeyFrames";
      AnimationDuration,      75,  simple,  "animation-duration",       "animationDuration";
      AnimationTimingFunction,76,  simple,  "animation-timing-function","animationTimingFunction";
      AnimationDelay,         77,  simple,  "animation-delay",          "animationDelay";
      AnimationIterationCount,78,  simple,  "animation-iteration-count","animationIterationCount";
      Content,                79,  simple,  "content",                  "content";
      WordBreak,              80,  simple,  "word-break",               "wordBreak";
      WebkitLineClamp,        81,  simple,  "-webkit-line-clamp",       "lineClamp", "webkitLineClamp";
      AnimationFillMode,      82,  simple,  "animation-fill-mode",      "animationFillMode";
      BackgroundPositionX,    83,  simple,  "background-position-x",    "backgroundPositionX";
      BackgroundPositionY,    84,  simple,  "background-position-y",    "backgroundPositionY";
      // 过渡
      Transition,             85,  compound,"transition",               "transition";
      TransitionProperty,     86,  simple,  "transition-property",      "transitionProperty";
      TransitionDuration,     87,  simple,  "transition-duration",      "transitionDuration";
      TransitionTimingFunction,88, simple,  "transition-timing-function","transitionTimingFunction";
      TransitionDelay,        89,  simple,  "transition-delay",         "transitionDelay";
      WhiteSpace,             90,  simple,  "white-space",              "whiteSpace";
      TextDecorationLine,     91,  simple,  "text-decoration-line",     "textDecorationLine";
      TextDecorationThickness,92,  simple,  "text-decoration-thickness","textDecorationThickness";
      TextDecorationStyle,    93,  simple,  "text-decoration-style",    "textDecorationStyle";
      TextDecorationColor,    94,  simple,  "text-decoration-color",    "textDecorationColor";
      AnimationName,          95,  simple,  "animation-name",           "animationName";
      BorderWidth,            96,  compound,"border-width",             "borderWidth";
      BorderColor,            97,  compound,"border-color",             "borderColor";
      Margin,                 98,  compound,"margin",                   "margin";
      Padding,                99,  compound,"padding",                  "padding";
      BorderRadius,           100, compound,"border-radius",            "borderRadius";
      BoxOrient,              101, simple,  "box-orient",               "boxOrient";
      PointerEvents,          102, simple,  "pointer-events",           "pointerEvents", "PointerEvents";
      Background,             103, compound,"background",               "background";
      Flex,                   104, compound,"flex",                     "flex";
      Border,                 105, compound,"border",                   "border";
      BorderStyle,            106, compound,"border-style",             "borderStyle";
      Gap,                    107, compound,"gap",                      "gap";
      AnimationDirection,     108, simple,  "animation-direction",      "animationDirection";
      AnimationPlayState,     109, simple,  "animation-play-state",     "animationPlayState";
      Animation,              110, compound,"animation",                "animation";
      Filter,                 111, simple,  "filter",                   "filter";
      BorderTop,              112, compound,"border-top",               "borderTop";
      BorderRight,            113, compound,"border-right",             "borderRight";
      BorderBottom,           114, compound,"border-bottom",            "borderBottom";
      BorderLeft,             115, compound,"border-left",              "borderLeft";
      TextUnderlineOffset,    116, simple,  "text-underline-offset",    "textUnderlineOffset";
      BackdropFilter,         117, simple,  "backdrop-filter",          "backdropFilter";
      // 网格布局（跨端基础集）
      GridTemplateColumns,    118, simple,  "grid-template-columns",    "gridTemplateColumns";
      GridTemplateRows,       119, simple,  "grid-template-rows",       "gridTemplateRows";
      GridRowStart,           120, simple,  "grid-row-start",           "gridRowStart";
      GridRowEnd,             121,  simple,  "grid-row-end",            "gridRowEnd";
      GridColumnStart,        122, simple,  "grid-column-start",        "gridColumnStart";
      GridColumnEnd,          123, simple,  "grid-column-end",          "gridColumnEnd";
      GridRow,                124, compound,"grid-row",                 "gridRow";
      GridColumn,             125, compound,"grid-column",              "gridColumn";
      GridArea,               126, compound,"grid-area",                "gridArea";
      JustifyItems,           127, simple,  "justify-items",            "justifyItems";
      JustifySelf,            128, simple,  "justify-self",             "justifySelf";
    }
  };
}
