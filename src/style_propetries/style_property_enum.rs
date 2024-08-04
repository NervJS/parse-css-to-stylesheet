#[repr(u32)]
#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum ArkUI_Alignment {
    /** Top start. */
    ARKUI_ALIGNMENT_TOP_START = 0,
    /** Top center. */
    ARKUI_ALIGNMENT_TOP,
    /** Top end. */
    ARKUI_ALIGNMENT_TOP_END,
    /** Vertically centered start. */
    ARKUI_ALIGNMENT_START,
    /** Horizontally and vertically centered. */
    ARKUI_ALIGNMENT_CENTER,
    /** Vertically centered end. */
    ARKUI_ALIGNMENT_END,
    /** Bottom start. */
    ARKUI_ALIGNMENT_BOTTOM_START,
    /** Horizontally centered on the bottom. */
    ARKUI_ALIGNMENT_BOTTOM,
    /** Bottom end. */
    ARKUI_ALIGNMENT_BOTTOM_END,
}

#[repr(u32)]
#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum ArkUI_ImageRepeat {
    /** The image is not repeatedly drawn. */
    ARKUI_IMAGE_REPEAT_NONE = 0,
    /** The image is repeatedly drawn only along the x-axis. */
    ARKUI_IMAGE_REPEAT_X,
    /** The image is repeatedly drawn only along the y-axis. */
    ARKUI_IMAGE_REPEAT_Y,
    /** The image is repeatedly drawn along both axes. */
    ARKUI_IMAGE_REPEAT_XY,
}

#[repr(u32)]
#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum ArkUI_ImageSize {
    /** The original image aspect ratio is retained. */
    ARKUI_IMAGE_SIZE_AUTO = 0,
    /** Default value. The image is scaled with its aspect ratio retained for both sides to be greater than or equal
     *  to the display boundaries. */
    ARKUI_IMAGE_SIZE_COVER,
    /** The image is scaled with its aspect ratio retained for the content to be completely displayed within the display
     *  boundaries. */
    ARKUI_IMAGE_SIZE_CONTAIN,
}

#[repr(u32)]
#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum ArkUI_BorderStyle {
    /** Solid border. */
    ARKUI_BORDER_STYLE_SOLID = 0,
    /** Dashed border. */
    ARKUI_BORDER_STYLE_DASHED,
    /** Dotted border. */
    ARKUI_BORDER_STYLE_DOTTED,
}

#[repr(u32)]
#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum ArkUI_FlexAlignment {
    /** The child components are aligned with the start edge of the main axis. */
    ARKUI_FLEX_ALIGNMENT_START = 1,
    /** The child components are aligned in the center of the main axis. */
    ARKUI_FLEX_ALIGNMENT_CENTER = 2,
    /** The child components are aligned with the end edge of the main axis. */
    ARKUI_FLEX_ALIGNMENT_END = 3,
    /** The child components are evenly distributed along the main axis. The space between any two adjacent components
     *  is the same. The first component is aligned with the main-start, and the last component is aligned with
     *  the main-end. */
    ARKUI_FLEX_ALIGNMENT_SPACE_BETWEEN = 6,
    /** The child components are evenly distributed along the main axis. The space between any two adjacent components
     *  is the same. The space between the first component and main-start, and that between the last component and
     *  cross-main are both half the size of the space between two adjacent components. */
    ARKUI_FLEX_ALIGNMENT_SPACE_AROUND = 7,
    /** The child components are evenly distributed along the main axis. The space between the first component
     *  and main-start, the space between the last component and main-end, and the space between any two adjacent
     *  components are the same. */
    ARKUI_FLEX_ALIGNMENT_SPACE_EVENLY = 8,
}

#[repr(u32)]
#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum ArkUI_FlexDirection {
    /** The child components are arranged in the same direction as the main axis runs along the rows. */
    ARKUI_FLEX_DIRECTION_ROW = 0,
    /** The child components are arranged in the same direction as the main axis runs down the columns. */
    ARKUI_FLEX_DIRECTION_COLUMN,
    /** The child components are arranged opposite to the <b>ROW</b> direction. */
    ARKUI_FLEX_DIRECTION_ROW_REVERSE,
    /** The child components are arranged opposite to the <b>COLUMN</b> direction. */
    ARKUI_FLEX_DIRECTION_COLUMN_REVERSE,
}

#[repr(u32)]
#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum ArkUI_FlexWrap {
    /** The child components in the flex container are arranged in a single line, and they cannot overflow. */
    ARKUI_FLEX_WRAP_NO_WRAP = 0,
    /** The child components in the flex container are arranged in multiple lines, and they may overflow. */
    ARKUI_FLEX_WRAP_WRAP,
    /** The child components in the flex container are reversely arranged in multiple lines, and they may overflow. */
    ARKUI_FLEX_WRAP_WRAP_REVERSE,
}

#[repr(u32)]
#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum ArkUI_FontStyle {
    /** Standard font style. */
    ARKUI_FONT_STYLE_NORMAL = 0,
    /** Italic font style. */
    ARKUI_FONT_STYLE_ITALIC,
}

#[repr(u32)]
#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum ArkUI_AnimationCurve {
    /** The animation speed keeps unchanged. */
    ARKUI_CURVE_LINEAR = 0,
    /** The animation starts slowly, accelerates, and then slows down towards the end. */
    ARKUI_CURVE_EASE,
    /** The animation starts at a low speed and then picks up speed until the end. */
    ARKUI_CURVE_EASE_IN,
    /** The animation ends at a low speed. */
    ARKUI_CURVE_EASE_OUT,
    /** The animation starts and ends at a low speed. */
    ARKUI_CURVE_EASE_IN_OUT,
    /** The animation uses the standard curve */
    ARKUI_CURVE_FAST_OUT_SLOW_IN,
    /** The animation uses the deceleration curve. */
    ARKUI_CURVE_LINEAR_OUT_SLOW_IN,
    /** The animation uses the acceleration curve. */
    ARKUI_CURVE_FAST_OUT_LINEAR_IN,
    /** The animation uses the extreme deceleration curve. */
    ARKUI_CURVE_EXTREME_DECELERATION,
    /** The animation uses the sharp curve. */
    ARKUI_CURVE_SHARP,
    /** The animation uses the rhythm curve. */
    ARKUI_CURVE_RHYTHM,
    /** The animation uses the smooth curve. */
    ARKUI_CURVE_SMOOTH,
    /** The animation uses the friction curve */
    ARKUI_CURVE_FRICTION,
}

#[repr(u32)]
#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum ArkUI_AnimationFillMode {
    // 动画未执行时不会将任何样式应用于目标，动画播放完成之后恢复初始默认状态。
    ARKUI_ANIMATION_FILL_MODE_NONE = 0,
    // 目标将保留动画执行期间最后一个关键帧的状态。
    ARKUI_ANIMATION_FILL_MODE_FORWARDS,
    // 动画将在应用于目标时立即应用第一个关键帧中定义的值，并在delay期间保留此值。
    ARKUI_ANIMATION_FILL_MODE_BACKWARDS,
    // 动画将遵循Forwards和Backwards的规则，从而在两个方向上扩展动画属性。
    ARKUI_ANIMATION_FILL_MODE_BOTH,
}

#[repr(u32)]
#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum ArkUI_FontWeight {
    /** 100 */
    ARKUI_FONT_WEIGHT_W100 = 0,
    /** 200 */
    ARKUI_FONT_WEIGHT_W200,
    /** 300 */
    ARKUI_FONT_WEIGHT_W300,
    /** 400 */
    ARKUI_FONT_WEIGHT_W400,
    /** 500 */
    ARKUI_FONT_WEIGHT_W500,
    /** 600 */
    ARKUI_FONT_WEIGHT_W600,
    /** 700 */
    ARKUI_FONT_WEIGHT_W700,
    /** 800 */
    ARKUI_FONT_WEIGHT_W800,
    /** 900 */
    ARKUI_FONT_WEIGHT_W900,
    /** The font weight is bold. */
    ARKUI_FONT_WEIGHT_BOLD,
    /** The font weight is normal. */
    ARKUI_FONT_WEIGHT_NORMAL,
    /** The font weight is bolder. */
    ARKUI_FONT_WEIGHT_BOLDER,
    /** The font weight is lighter. */
    ARKUI_FONT_WEIGHT_LIGHTER,
    /** The font weight is medium. */
    ARKUI_FONT_WEIGHT_MEDIUM,
    /** The font weight is normal. */
    ARKUI_FONT_WEIGHT_REGULAR,
}

#[repr(u32)]
#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum ArkUI_ItemAlignment {
    /** The default configuration in the container is used. */
    ARKUI_ITEM_ALIGNMENT_AUTO = 0,
    /** The items in the container are aligned with the cross-start edge. */
    ARKUI_ITEM_ALIGNMENT_START,
    /** The items in the container are centered along the cross axis. */
    ARKUI_ITEM_ALIGNMENT_CENTER,
    /** The items in the container are aligned with the cross-end edge. */
    ARKUI_ITEM_ALIGNMENT_END,
    /** The items in the container are stretched and padded along the cross axis. */
    ARKUI_ITEM_ALIGNMENT_STRETCH,
    /** The items in the container are aligned in such a manner that their text baselines are aligned along the
     *  cross axis. */
    ARKUI_ITEM_ALIGNMENT_BASELINE,
}

#[repr(u32)]
#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum ArkUI_TextAlignment {
    /** Aligned with the start. */
    ARKUI_TEXT_ALIGNMENT_START = 0,
    /** Horizontally centered. */
    ARKUI_TEXT_ALIGNMENT_CENTER,
    /** Aligned with the end. */
    ARKUI_TEXT_ALIGNMENT_END,
    /** Aligned with both margins. */
    ARKUI_TEXT_ALIGNMENT_JUSTIFY,
}

#[repr(u32)]
#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum ArkUI_TextDecorationType {
    /** No text decoration. */
    ARKUI_TEXT_DECORATION_TYPE_NONE = 0,
    /** Line under the text. */
    ARKUI_TEXT_DECORATION_TYPE_UNDERLINE,
    /** Line over the text. */
    ARKUI_TEXT_DECORATION_TYPE_OVERLINE,
    /** Line through the text. */
    ARKUI_TEXT_DECORATION_TYPE_LINE_THROUGH,
}

#[repr(u32)]
#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum ArkUI_TextDecorationStyle {
    /** Single solid line. */
    ARKUI_TEXT_DECORATION_STYLE_SOLID = 0,
    /** Double solid line. */
    ARKUI_TEXT_DECORATION_STYLE_DOUBLE,
    /** Dotted line. */
    ARKUI_TEXT_DECORATION_STYLE_DOTTED,
    /** Dashed line. */
    ARKUI_TEXT_DECORATION_STYLE_DASHED,
    /** Wavy line. */
    ARKUI_TEXT_DECORATION_STYLE_WAVY,
}

#[repr(u32)]
#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum ArkUI_TextOverflow {
    /** Extra-long text is not clipped. */
    ARKUI_TEXT_OVERFLOW_NONE = 0,
    /** Extra-long text is clipped. */
    ARKUI_TEXT_OVERFLOW_CLIP,
    /** An ellipsis (...) is used to represent text overflow. */
    ARKUI_TEXT_OVERFLOW_ELLIPSIS,
    /** Text continuously scrolls when text overflow occurs. */
    ARKUI_TEXT_OVERFLOW_MARQUEE,
}

#[repr(u32)]
#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum ArkUI_TextCase {
    /** The original case of the text is retained. */
    ARKUI_TEXT_CASE_NORMAL = 0,
    /** All letters in the text are in lowercase. */
    ARKUI_TEXT_CASE_LOWER,
    /** All letters in the text are in uppercase. */
    ARKUI_TEXT_CASE_UPPER,
}

#[repr(u32)]
#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum ArkUI_LinearGradientDirection {
    /** From right to left. */
    ARKUI_LINEAR_GRADIENT_DIRECTION_LEFT = 0,
    /** From bottom to top. */
    ARKUI_LINEAR_GRADIENT_DIRECTION_TOP,
    /** From left to right. */
    ARKUI_LINEAR_GRADIENT_DIRECTION_RIGHT,
    /** From top to bottom. */
    ARKUI_LINEAR_GRADIENT_DIRECTION_BOTTOM,
    /** From lower right to upper left. */
    ARKUI_LINEAR_GRADIENT_DIRECTION_LEFT_TOP,
    /** From upper right to lower left. */
    ARKUI_LINEAR_GRADIENT_DIRECTION_LEFT_BOTTOM,
    /** From lower left to upper right. */
    ARKUI_LINEAR_GRADIENT_DIRECTION_RIGHT_TOP,
    /** From upper left to lower right. */
    ARKUI_LINEAR_GRADIENT_DIRECTION_RIGHT_BOTTOM,
    /** No gradient. */
    ARKUI_LINEAR_GRADIENT_DIRECTION_NONE,
    /** Custom direction. */
    ARKUI_LINEAR_GRADIENT_DIRECTION_CUSTOM,
}

#[repr(u32)]
#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum ArkUI_Visibility {
    /** The component is visible. */
    ARKUI_VISIBILITY_VISIBLE = 0,
    /** The component is hidden, and a placeholder is used for it in the layout. */
    ARKUI_VISIBILITY_HIDDEN,
    /** The component is hidden. It is not involved in the layout, and no placeholder is used for it. */
    ARKUI_VISIBILITY_NONE,
}

#[repr(u32)]
#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum ArkUI_WordBreak {
    /** Word breaks can occur between any two characters for Chinese, Japanese, and Korean (CJK) text, but can occur
     *  only at a space character for non-CJK text (such as English). */
    ARKUI_WORD_BREAK_NORMAL = 0,
    /** Word breaks can occur between any two characters for non-CJK text. CJK text behavior is the same as for
     *  <b>NORMAL</b>. */
    ARKUI_WORD_BREAK_BREAK_ALL,
    /** This option has the same effect as <b>BREAK_ALL</b> for non-CJK text, except that if it preferentially wraps
    *  lines at appropriate characters (for example, spaces) whenever possible.
       CJK text behavior is the same as for <b>NORMAL</b>. */
    ARKUI_WORD_BREAK_BREAK_WORD,
}

#[repr(u32)]
#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum PlaceholderVerticalAlignment {
    /** Offset At Baseline */
    ALIGNMENT_OFFSET_AT_BASELINE = 0,
    /** Above Baseline */
    ALIGNMENT_ABOVE_BASELINE,
    /** Below Baseline */
    ALIGNMENT_BELOW_BASELINE,
    /** Top of Row Box */
    ALIGNMENT_TOP_OF_ROW_BOX,
    /** Bottom of Row Box */
    ALIGNMENT_BOTTOM_OF_ROW_BOX,
    /** Center of Row Box */
    ALIGNMENT_CENTER_OF_ROW_BOX,
}

#[repr(u32)]
#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub enum Overflow {
    Visible = 0,
    Hidden,
    Scroll,
}

#[repr(u32)]
#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub enum Display {
    None = 0,
    Block,
    Flex,
    Box,
}

#[repr(u32)]
#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub enum Position {
    Static = 0,
    Relative,
    Absolute,
    Fixed,
    Sticky,
}


#[repr(u32)]
#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub enum WhiteSpace {
    NoWrap = 0,
    Wrap,
}

#[repr(u32)]
#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub enum BoxOrient {
    Horizontal = 0,
    Vertical,
    InlineAxis,
    BlockAxis,
}