## [1.1.39](https://github.com/NervJS/parse-css-to-stylesheet/compare/v1.1.37...v1.1.39) (2026-03-17)


### Bug Fixes

* 修复white-space的normal不生效 ([f6167fd](https://github.com/NervJS/parse-css-to-stylesheet/commit/f6167fdc3b8d4855576b2395499ceb0659ead2cf))
* add publishConfig to win32-x64-msvc package ([36c0b2e](https://github.com/NervJS/parse-css-to-stylesheet/commit/36c0b2e45dc270075c84059f963c1aa739c1f2ca))



## [1.1.38](https://github.com/NervJS/parse-css-to-stylesheet/compare/v1.1.37...v1.1.38) (2026-03-17)


### Bug Fixes

* 修复white-space的normal不生效 ([f6167fd](https://github.com/NervJS/parse-css-to-stylesheet/commit/f6167fdc3b8d4855576b2395499ceb0659ead2cf))



## [1.1.37](https://github.com/NervJS/parse-css-to-stylesheet/compare/v1.1.36...v1.1.37) (2026-01-26)


### Bug Fixes

* 修复box-orient解析异常 ([4117a94](https://github.com/NervJS/parse-css-to-stylesheet/commit/4117a9412df302547f737ef374ed34c050f49ee5))



## [1.1.36](https://github.com/NervJS/parse-css-to-stylesheet/compare/v1.1.35...v1.1.36) (2026-01-09)


### Features

* 支持stikcy解析 ([2ddcdb5](https://github.com/NervJS/parse-css-to-stylesheet/commit/2ddcdb5a37a2c685e47fe9dce3f48229cf6a4e2d))



## [1.1.35](https://github.com/NervJS/parse-css-to-stylesheet/compare/v1.1.34...v1.1.35) (2025-11-10)


### Features

* 支持border-left/right/bottom/top 4个方向的变量处理 ([2dba431](https://github.com/NervJS/parse-css-to-stylesheet/commit/2dba43183c69cdcfe586817152d65cfbf7b3cc00))



## [1.1.34](https://github.com/NervJS/parse-css-to-stylesheet/compare/v1.1.33...v1.1.34) (2025-11-05)



## [1.1.33](https://github.com/NervJS/parse-css-to-stylesheet/compare/v1.1.32...v1.1.33) (2025-11-03)


### Bug Fixes

* 修复font-family解析失败导致编译异常 ([f2e046d](https://github.com/NervJS/parse-css-to-stylesheet/commit/f2e046d19438270dbb4a71695151f6d316949f65))



## [1.1.32](https://github.com/NervJS/parse-css-to-stylesheet/compare/v1.1.31...v1.1.32) (2025-10-31)


### Bug Fixes

* 修复font-family处理.的转义 ([8c80e91](https://github.com/NervJS/parse-css-to-stylesheet/commit/8c80e91a2bdbe468895516940b9d1de53619ecb8))



## [1.1.31](https://github.com/NervJS/parse-css-to-stylesheet/compare/v1.1.30...v1.1.31) (2025-10-31)



## [1.1.30](https://github.com/NervJS/parse-css-to-stylesheet/compare/v1.1.29...v1.1.30) (2025-10-31)


### Bug Fixes

* 修复选择器被合并导致匹配异常、修复fontfamliy不能有数字 ([f12d632](https://github.com/NervJS/parse-css-to-stylesheet/commit/f12d632af44518185de0781913926856a4ee1f4a))


### Features

* 锁定cargo的依赖版本 ([3fbbe99](https://github.com/NervJS/parse-css-to-stylesheet/commit/3fbbe997f89bf4e6095a42a6494c46bf52863d62))



## [1.1.29](https://github.com/NervJS/parse-css-to-stylesheet/compare/v1.1.28...v1.1.29) (2025-10-11)


### Features

* transition 为 all的时候，解析成CSSPropertyType::All而不是原来是-1 ([791cd84](https://github.com/NervJS/parse-css-to-stylesheet/commit/791cd846c4c547bca07ccdf892f65a3b17334a13))



## [1.1.28](https://github.com/NervJS/parse-css-to-stylesheet/compare/v1.1.27...v1.1.28) (2025-09-10)


### Features

* 增加滤镜解析 ([13c33bf](https://github.com/NervJS/parse-css-to-stylesheet/commit/13c33bf0c23aa5d119384ac114798e27df019f7a))



## [1.1.27](https://github.com/NervJS/parse-css-to-stylesheet/compare/v1.1.26...v1.1.27) (2025-09-09)


### Bug Fixes

* 修复媒体查询的单位范围 ([7ef6c73](https://github.com/NervJS/parse-css-to-stylesheet/commit/7ef6c739d6828ad443b768b3871511562dffa34e))



## [1.1.26](https://github.com/NervJS/parse-css-to-stylesheet/compare/v1.1.25...v1.1.26) (2025-08-21)


### Bug Fixes

* 修复:root不生效 ([5b0ae7d](https://github.com/NervJS/parse-css-to-stylesheet/commit/5b0ae7d707a153da503e4d25a969f9b91e4d5242))



## [1.1.25](https://github.com/NervJS/parse-css-to-stylesheet/compare/v1.1.23...v1.1.25) (2025-08-19)


### Bug Fixes

* 修复纯数字解析负数异常 ([02013b4](https://github.com/NervJS/parse-css-to-stylesheet/commit/02013b46d58493a920aa8113cdbc9feb259f51bf))


### Features

* 当json是一个空数组的时候，fb也转成一个空数组 ([33f7133](https://github.com/NervJS/parse-css-to-stylesheet/commit/33f7133701a148016cee0fbae95e056db60d6167))



## [1.1.24](https://github.com/NervJS/parse-css-to-stylesheet/compare/v1.1.23...v1.1.24) (2025-08-12)


### Features

* 当json是一个空数组的时候，fb也转成一个空数组 ([33f7133](https://github.com/NervJS/parse-css-to-stylesheet/commit/33f7133701a148016cee0fbae95e056db60d6167))



## [1.1.23](https://github.com/NervJS/parse-css-to-stylesheet/compare/v1.1.22...v1.1.23) (2025-08-11)


### Features

* 补充测试用例 ([c6d844a](https://github.com/NervJS/parse-css-to-stylesheet/commit/c6d844a30fc40b933cb70488ef609127614dabd6))
* 处理css变量，变成lpx ([aa9348f](https://github.com/NervJS/parse-css-to-stylesheet/commit/aa9348f09d781593b927072b460669cff98eb951))
* remove log ([52ba24b](https://github.com/NervJS/parse-css-to-stylesheet/commit/52ba24b6c69dcdab260251b88358fd2668284dd0))



## [1.1.22](https://github.com/NervJS/parse-css-to-stylesheet/compare/v1.1.21...v1.1.22) (2025-07-01)


### Bug Fixes

* 修复calc解析vw异常 ([33b509e](https://github.com/NervJS/parse-css-to-stylesheet/commit/33b509ef5db667a75e3a78456787c8d2359e78db))



## [1.1.21](https://github.com/NervJS/parse-css-to-stylesheet/compare/v1.1.20...v1.1.21) (2025-06-26)



## [1.1.20](https://github.com/NervJS/parse-css-to-stylesheet/compare/v1.1.19...v1.1.20) (2025-06-26)


### Features

* 支持设计稿转换模式配置 ([48bc103](https://github.com/NervJS/parse-css-to-stylesheet/commit/48bc10376be711421437c3f2719c2cbf048cc4b9))



## [1.1.19](https://github.com/NervJS/parse-css-to-stylesheet/compare/v1.1.18...v1.1.19) (2025-06-19)


### Features

* 添加 animation-fill-mode 的解析 ([b41ff1b](https://github.com/NervJS/parse-css-to-stylesheet/commit/b41ff1bab9a45f303881b7f037c818971f07f314))



## [1.1.18](https://github.com/NervJS/parse-css-to-stylesheet/compare/v1.1.17...v1.1.18) (2025-06-10)


### Features

* 增加display: inline和inline-block ([cecfd33](https://github.com/NervJS/parse-css-to-stylesheet/commit/cecfd3364d1ef376ba689bf34d6f29b22410e2a1))



## [1.1.17](https://github.com/NervJS/parse-css-to-stylesheet/compare/v1.1.16...v1.1.17) (2025-05-30)


### Bug Fixes

* 修复在postcss兼容低版本浏览器的额外样式 ([c0771a6](https://github.com/NervJS/parse-css-to-stylesheet/commit/c0771a64ea7004988139cb7f1c7909703b37087f))


### Features

* 添加解析bin文件的命令 ([911569b](https://github.com/NervJS/parse-css-to-stylesheet/commit/911569b836c957a0350f37d6ff2c2321e615a8f8))



## [1.1.16](https://github.com/NervJS/parse-css-to-stylesheet/compare/v1.1.15...v1.1.16) (2025-05-20)


### Features

* 添加animationDirection和animationplaystate的解析 ([e0d028d](https://github.com/NervJS/parse-css-to-stylesheet/commit/e0d028dd85e29ad354955ac3d85fd2df81d5452f))



## [1.1.15](https://github.com/NervJS/parse-css-to-stylesheet/compare/v1.1.14...v1.1.15) (2025-05-14)


### Features

* 更新stylesheet_generated ([5095f15](https://github.com/NervJS/parse-css-to-stylesheet/commit/5095f1586103d8a342026d2eb9d3052fc16e3f10))
* 修复calc对于px字符串的处理 ([bbc5539](https://github.com/NervJS/parse-css-to-stylesheet/commit/bbc5539f33f05b332f3afd75dd243023d0b0a6a3))



## [1.1.14](https://github.com/NervJS/parse-css-to-stylesheet/compare/v1.1.13...v1.1.14) (2025-05-01)


### Features

* 添加keyframse的flatbuffer解析 ([fa2cef9](https://github.com/NervJS/parse-css-to-stylesheet/commit/fa2cef940b33b0804ab780e1ced8ac254b5d7080))



## [1.1.13](https://github.com/NervJS/parse-css-to-stylesheet/compare/v1.1.12...v1.1.13) (2025-04-29)


### Features

* 修复bg的转义词，增加allow_inherit的配置 ([a179cfc](https://github.com/NervJS/parse-css-to-stylesheet/commit/a179cfcad37a7f798c4452071dd92e926875de40))



## [1.1.12](https://github.com/NervJS/parse-css-to-stylesheet/compare/v1.1.11...v1.1.12) (2025-03-31)


### Features

* 增加flatbuffer的variables字段 ([e66c19b](https://github.com/NervJS/parse-css-to-stylesheet/commit/e66c19b9213ad2f67e27f565adb5d6b71160d721))



## [1.1.11](https://github.com/NervJS/parse-css-to-stylesheet/compare/v1.1.10...v1.1.11) (2025-03-31)


### Features

* 修改flatbuffer产物编译 ([57a106f](https://github.com/NervJS/parse-css-to-stylesheet/commit/57a106f3fc50112a38f9fe7b55c64c152c78bc6b))



## [1.1.10](https://github.com/NervJS/parse-css-to-stylesheet/compare/v1.1.9...v1.1.10) (2025-03-25)


### Features

* 增加 pseudo_key ([d176917](https://github.com/NervJS/parse-css-to-stylesheet/commit/d176917637049a32341d95c2268a5e9cbc8d6df3))



## [1.1.9](https://github.com/NervJS/parse-css-to-stylesheet/compare/v1.1.8...v1.1.9) (2025-03-22)


### Features

* 生成二进制文件时支持伪类 ([d1aed1c](https://github.com/NervJS/parse-css-to-stylesheet/commit/d1aed1c8ed231dbeb61ca7e781dcf048396d798a))



## [1.1.8](https://github.com/NervJS/parse-css-to-stylesheet/compare/v1.1.7...v1.1.8) (2025-03-13)


### Features

* 增加对important的识别 ([08b378d](https://github.com/NervJS/parse-css-to-stylesheet/commit/08b378d2eda6c3e4a7719b11520106cdaa1c684f))



## [1.1.8](https://github.com/NervJS/parse-css-to-stylesheet/compare/v1.1.7...v1.1.8) (2025-03-13)


### Features

* 增加对important的识别 ([08b378d](https://github.com/NervJS/parse-css-to-stylesheet/commit/08b378d2eda6c3e4a7719b11520106cdaa1c684f))



## [1.1.8](https://github.com/NervJS/parse-css-to-stylesheet/compare/v1.1.7...v1.1.8) (2025-03-13)


### Features

* 增加对important的识别 ([08b378d](https://github.com/NervJS/parse-css-to-stylesheet/commit/08b378d2eda6c3e4a7719b11520106cdaa1c684f))



## [1.1.7](https://github.com/NervJS/parse-css-to-stylesheet/compare/v1.1.6...v1.1.7) (2025-02-27)


### Features

* 增加暗黑模式的监听，优化产物体积 ([318996a](https://github.com/NervJS/parse-css-to-stylesheet/commit/318996a87f16491cb8744565c168030c3adde0ab))



## [1.1.6](https://github.com/NervJS/parse-css-to-stylesheet/compare/v1.1.5...v1.1.6) (2025-02-24)


### Features

* 增加css变量的解析 ([86cae60](https://github.com/NervJS/parse-css-to-stylesheet/commit/86cae60ef1930b15119abb4244b9d27b6cf08b74))



## [1.1.5](https://github.com/NervJS/parse-css-to-stylesheet/compare/v1.1.4...v1.1.5) (2025-02-19)



## [1.1.4](https://github.com/NervJS/parse-css-to-stylesheet/compare/v1.1.3...v1.1.4) (2025-02-19)


### Features

* 增加对规则对env的提前识别 ([a842ecb](https://github.com/NervJS/parse-css-to-stylesheet/commit/a842ecbd884b262858e83232beb69997d20fd804))



## [1.1.3](https://github.com/NervJS/parse-css-to-stylesheet/compare/v1.1.2...v1.1.3) (2025-02-13)


### Features

* 生成二进制文件时支持处理 fonts 和 medias ([daff126](https://github.com/NervJS/parse-css-to-stylesheet/commit/daff126a1165bb3c28680d4194d0d5c9d2923b87))



## [1.1.2](https://github.com/NervJS/parse-css-to-stylesheet/compare/v1.1.1...v1.1.2) (2025-01-10)


### Bug Fixes

* 数值转换时最多保留两位小数，同时修复数值精确度问题 ([5630b70](https://github.com/NervJS/parse-css-to-stylesheet/commit/5630b707c5378f11b856364f1e8314acd557ae55))



## [1.1.1](https://github.com/NervJS/parse-css-to-stylesheet/compare/v1.1.0...v1.1.1) (2024-12-25)


### Features

* 生成样式中增加 design_width 设计稿宽度 ([82a7308](https://github.com/NervJS/parse-css-to-stylesheet/commit/82a73085414921cb288a0acae5430fa6e2b2e955))



# [1.1.0](https://github.com/NervJS/parse-css-to-stylesheet/compare/v1.0.20...v1.1.0) (2024-12-03)


### Bug Fixes

* 生成二进制暂时不处理 fonts/media/keyframes ([6f4a2d9](https://github.com/NervJS/parse-css-to-stylesheet/commit/6f4a2d94234349db2162d7bd2a0e14182e7ad2b6))
* 完善样式编译成二进制处理 ([f0c467e](https://github.com/NervJS/parse-css-to-stylesheet/commit/f0c467edb3db331509661a69133a7fa94c186d7c))
* update ci workflow ([c0a87a6](https://github.com/NervJS/parse-css-to-stylesheet/commit/c0a87a695cc000131282cd850b96036e9e7a98aa))


### Features

* 更新 typings ([098e3d2](https://github.com/NervJS/parse-css-to-stylesheet/commit/098e3d23aba86b69ac277b655bd42bdc62d05e50))
* 添加pointer_events的解析 ([24f9746](https://github.com/NervJS/parse-css-to-stylesheet/commit/24f9746b475caa07c9baceab5ec525852ec7fbc4))
* 支持将样式编译成二进制文件 ([8d53ada](https://github.com/NervJS/parse-css-to-stylesheet/commit/8d53adabd75a9e37033cfcffa2a6c0bc4df3d52c))
* add registry ([f4b8ce7](https://github.com/NervJS/parse-css-to-stylesheet/commit/f4b8ce76a15f22442105fa8a2b5fd2718722170b))
* rm node 16 ([ec7dbfc](https://github.com/NervJS/parse-css-to-stylesheet/commit/ec7dbfcb6d4edd96b9f16fdc1dbd2511ff103702))
* use checkout v4 ([859aa90](https://github.com/NervJS/parse-css-to-stylesheet/commit/859aa90b673a3ac254be38b9bdfcde0c12824e9f))
* use npm test ([8b72c6b](https://github.com/NervJS/parse-css-to-stylesheet/commit/8b72c6b69444f9a4c4055cb0238803b666d69711))
* use pnpm ([1365797](https://github.com/NervJS/parse-css-to-stylesheet/commit/136579758f344b1a34af19f8e58bf7382064ddf6))
* use pnpm ci ([66f1f9e](https://github.com/NervJS/parse-css-to-stylesheet/commit/66f1f9e05768d59104dbc1e7e0ddc4f1ea049697))



## [1.0.20](https://github.com/NervJS/parse-css-to-stylesheet/compare/v1.0.19...v1.0.20) (2024-08-17)


### Features

* 支持解析 font-family ([5d19340](https://github.com/NervJS/parse-css-to-stylesheet/commit/5d19340e51e1b6fc8fd763b2bdaaa7e6099aaf92))



## [1.0.19](https://github.com/NervJS/parse-css-to-stylesheet/compare/v1.0.18...v1.0.19) (2024-08-15)


### Features

* 字体解析路径 ([63213f3](https://github.com/NervJS/parse-css-to-stylesheet/commit/63213f3a65facb09b70de754d2850781db799d53))



## [1.0.18](https://github.com/NervJS/parse-css-to-stylesheet/compare/v1.0.17...v1.0.18) (2024-08-12)


### Features

* 支持解析字体 ([e15d053](https://github.com/NervJS/parse-css-to-stylesheet/commit/e15d053abea4acbeedfb1e14b3d558c1a2496a6b))



## [1.0.17](https://github.com/NervJS/parse-css-to-stylesheet/compare/v1.0.16...v1.0.17) (2024-08-04)


### Features

* 增加 box-orient 解析 && display 增加 -webkit-box 解析 ([fd8fa60](https://github.com/NervJS/parse-css-to-stylesheet/commit/fd8fa60ef4a838bbff4a2fc310d71a8da72a490f))



## [1.0.16](https://github.com/NervJS/parse-css-to-stylesheet/compare/v1.0.13...v1.0.16) (2024-07-30)


### Bug Fixes

* 修复测试案例 ([7615133](https://github.com/NervJS/parse-css-to-stylesheet/commit/76151336ce95fda182cdc2ffe1f0422ec5877f8d))


### Features

* 加入媒体查询 ([f3c7ab2](https://github.com/NervJS/parse-css-to-stylesheet/commit/f3c7ab227c7144d11a23ae20f118b39d6a30a194))
* 将媒体查询下Resolution单位🙆统一 ([c225926](https://github.com/NervJS/parse-css-to-stylesheet/commit/c225926614c7ab3c82ec6018d27907d2b11eb284))
* 修改resolution为dpi的单位 ([e16092e](https://github.com/NervJS/parse-css-to-stylesheet/commit/e16092e7a6006f9a4ebcec8e46913a6d5cad4fc3))
* css支持多animation ([21bc8a9](https://github.com/NervJS/parse-css-to-stylesheet/commit/21bc8a92b94b94f043ba98563411dc4d348af75d))



## [1.0.13](https://github.com/NervJS/parse-css-to-stylesheet/compare/v1.0.12...v1.0.13) (2024-07-22)


### Bug Fixes

* 修复webkit适配错误，background:none\border:none解析异常 ([75c5b2e](https://github.com/NervJS/parse-css-to-stylesheet/commit/75c5b2ea91898a937bd240a669109381a51a3cdd))



## [1.0.12](https://github.com/NervJS/parse-css-to-stylesheet/compare/v1.0.11...v1.0.12) (2024-07-17)


### Features

* 增加对env的解析 ([c860d55](https://github.com/NervJS/parse-css-to-stylesheet/commit/c860d5583a8fd8ec3652b8f8453772fc89f06d1e))



## [1.0.11](https://github.com/NervJS/parse-css-to-stylesheet/compare/v1.0.10...v1.0.11) (2024-07-17)


### Features

* 去掉line_height打印 ([ec7c98d](https://github.com/NervJS/parse-css-to-stylesheet/commit/ec7c98d5463ea6dc335686dc869ee24cc486ba34))



## [1.0.10](https://github.com/NervJS/parse-css-to-stylesheet/compare/v1.0.9...v1.0.10) (2024-07-16)


### Bug Fixes

* 修复顺序问题 ([d23ccb6](https://github.com/NervJS/parse-css-to-stylesheet/commit/d23ccb6c8f304ccc288c56601d71efd214647403))



## [1.0.9](https://github.com/NervJS/parse-css-to-stylesheet/compare/v1.0.8...v1.0.9) (2024-07-16)


### Bug Fixes

* 修复visibility解析成display的bug ([21564cf](https://github.com/NervJS/parse-css-to-stylesheet/commit/21564cff375dc5fdb42496bf5afdf023347853af))


### Features

* line_height把数字转成百分比 ([ce8303f](https://github.com/NervJS/parse-css-to-stylesheet/commit/ce8303f5ea321ceeeffecdaa95b397e8966a0ed5))



## [1.0.8](https://github.com/NervJS/parse-css-to-stylesheet/compare/v1.0.7...v1.0.8) (2024-07-10)


### Bug Fixes

* 修复background无法解析复合的background-position ([5501f95](https://github.com/NervJS/parse-css-to-stylesheet/commit/5501f95dd3ce659c4c4bc5b65272b622a6825234))



## [1.0.7](https://github.com/NervJS/parse-css-to-stylesheet/compare/v1.0.6...v1.0.7) (2024-07-09)


### Features

* transition解析 ([e718001](https://github.com/NervJS/parse-css-to-stylesheet/commit/e718001efbf94384370e86f5e92ba2fa73b807fb))



## [1.0.6](https://github.com/NervJS/parse-css-to-stylesheet/compare/v1.0.5...v1.0.6) (2024-07-04)


### Features

* background枚举变成百分比 ([bb34cb3](https://github.com/NervJS/parse-css-to-stylesheet/commit/bb34cb32149cbb9fd450c20b0f77d5abed2ec5ab))



## [1.0.5](https://github.com/NervJS/parse-css-to-stylesheet/compare/v1.0.4...v1.0.5) (2024-07-03)


### Features

* 拆分text-decoration ([ee55b97](https://github.com/NervJS/parse-css-to-stylesheet/commit/ee55b970f22777f1981aa8f16b73a499c1ce048f))



## [1.0.4](https://github.com/NervJS/parse-css-to-stylesheet/compare/v1.0.3...v1.0.4) (2024-07-02)


### Features

* 增加white-space ([f378247](https://github.com/NervJS/parse-css-to-stylesheet/commit/f378247b5f1b09c52df3cc5c2b37fffae7a86e8b))
* background-postion支持百分比 ([946e38f](https://github.com/NervJS/parse-css-to-stylesheet/commit/946e38f2709c2273b5bfc63525a1a7709a7cd34e))



## [1.0.3](https://github.com/NervJS/parse-css-to-stylesheet/compare/v1.0.1...v1.0.3) (2024-06-25)


### Bug Fixes

* 修改enum变量名 ([aae2850](https://github.com/NervJS/parse-css-to-stylesheet/commit/aae28501ba30f918e2e1fe38efe57bfb26d1b8a8))


### Features

* 把background-position拆成background-positionx，background-positiony ([c450d8f](https://github.com/NervJS/parse-css-to-stylesheet/commit/c450d8fe13806892295a2919f0c0506fed6d9d7c))
* 去掉xyz的引号 ([fa4d75a](https://github.com/NervJS/parse-css-to-stylesheet/commit/fa4d75a63b8ba49b828a22860f1beeb05b00687c))
* transition的enum ([961c89f](https://github.com/NervJS/parse-css-to-stylesheet/commit/961c89f85e986c2702145ccbad632cc2ab2bd59e))



## [1.0.1](https://github.com/NervJS/parse-css-to-stylesheet/compare/v1.0.0...v1.0.1) (2024-06-18)


### Bug Fixes

* 修改vertical-align的映射 ([9bc3735](https://github.com/NervJS/parse-css-to-stylesheet/commit/9bc37353b0fcc80c10eeaa0ca33720ac8262f337))



# [1.0.0](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.69...v1.0.0) (2024-06-17)


### Bug Fixes

* 修复poistion的枚举错误问题 ([5700315](https://github.com/NervJS/parse-css-to-stylesheet/commit/570031538ad6257e4b53522d6c3a731349b21f70))


### Features

* 把fill mode解析成string ([e01ae73](https://github.com/NervJS/parse-css-to-stylesheet/commit/e01ae7329dd7858cda93a0f5000e8863045e3cc1))
* 更新 ([82a8f69](https://github.com/NervJS/parse-css-to-stylesheet/commit/82a8f69c25d5ce196b9ef135e39c9ae01ab2d28d))
* 关闭animation的枚举值解析 ([30e7792](https://github.com/NervJS/parse-css-to-stylesheet/commit/30e77927310a5384e9e1f7fd23bc3c45d4f1b67b))
* 解析单个的fill mode ([eaa995e](https://github.com/NervJS/parse-css-to-stylesheet/commit/eaa995e6e2ec183cb23dd9b3dcb412a1dbc6d1f5))
* 解析timingFunction ([f5f17ba](https://github.com/NervJS/parse-css-to-stylesheet/commit/f5f17ba30b2825d068566e09b05fb053f4f12fc6))
* 去掉打印 ([cc5f9a2](https://github.com/NervJS/parse-css-to-stylesheet/commit/cc5f9a2244e255e5aef9b6c92f198aac391eb8e1))
* 属性枚举化 ([67810ef](https://github.com/NervJS/parse-css-to-stylesheet/commit/67810ef4c5a875834670be8dfb03b9109abaea05))
* 添加animation fill mode ([a9f56f6](https://github.com/NervJS/parse-css-to-stylesheet/commit/a9f56f61c373dee57bddfc81d8abf41de1dd6524))
* 完成枚举化改造 ([7044391](https://github.com/NervJS/parse-css-to-stylesheet/commit/7044391384c7ac82d179d2c3d1f4cc7ed6c423b2))
* 完成word_break ([a3ca5e7](https://github.com/NervJS/parse-css-to-stylesheet/commit/a3ca5e77e20966da0815424af86dc1b368a87b6f))
* 修改 ([f021b1a](https://github.com/NervJS/parse-css-to-stylesheet/commit/f021b1a38b425a7baf370daed83bad22b48b64d4))
* 修改textoverflow ([358c3e2](https://github.com/NervJS/parse-css-to-stylesheet/commit/358c3e2c9b875053c385be5b350156508d4c696b))
* 增加部分伪类的枚举 ([493a427](https://github.com/NervJS/parse-css-to-stylesheet/commit/493a427f56c2955b19d3cf51d731ef5f45b3045b))
* 重构代码输出 ([65c7b32](https://github.com/NervJS/parse-css-to-stylesheet/commit/65c7b32a5784a94921b37e584a87016759c4b826))
* capi ([7a73ef7](https://github.com/NervJS/parse-css-to-stylesheet/commit/7a73ef7ca8f3335ebb3c29d2ebc630b27ad455b8))
* transform ([e5903da](https://github.com/NervJS/parse-css-to-stylesheet/commit/e5903da91e39f727d1b2d459d5a2ecfb19bd2861))



## [0.0.69](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.68...v0.0.69) (2024-05-14)


### Features

* 修改display，支持block的输出 ([7d1d995](https://github.com/NervJS/parse-css-to-stylesheet/commit/7d1d9950cee24e04e77bdbc5d6d2fe071f8a205a))



## [0.0.68](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.67...v0.0.68) (2024-05-09)


### Bug Fixes

* 修复自定义组件taro无样式问题 ([423b642](https://github.com/NervJS/parse-css-to-stylesheet/commit/423b642e8a28816ca40643d36d8b4fb748214fcf))



## [0.0.67](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.66...v0.0.67) (2024-04-29)



## [0.0.66](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.64...v0.0.66) (2024-04-29)



## [0.0.64](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.63...v0.0.64) (2024-04-29)


### Bug Fixes

* 修复box-shadow报错 ([9f88dd2](https://github.com/NervJS/parse-css-to-stylesheet/commit/9f88dd21e77544f18b4fa7f1522d15cc76686f86))



## [0.0.63](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.60...v0.0.63) (2024-04-28)


### Features

* 支持box-shadow ([eeecd38](https://github.com/NervJS/parse-css-to-stylesheet/commit/eeecd38b0d741ed009d4cf1e80be80c35ad082eb))



## [0.0.60](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.59...v0.0.60) (2024-04-24)


### Features

* 增加对复杂的createElement支持 ([d31dca1](https://github.com/NervJS/parse-css-to-stylesheet/commit/d31dca1603de1bb4d1ebfb90a0d65eeedb494514))



## [0.0.59](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.58...v0.0.59) (2024-04-23)


### Features

* 移除无用的导入 ([af9e343](https://github.com/NervJS/parse-css-to-stylesheet/commit/af9e343d82906ec1e82b3fd22b90acc7ccd11252))



## [0.0.58](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.57...v0.0.58) (2024-04-22)


### Features

* 支持empty选择器，支持解析createElement嵌套样式 ([38e6856](https://github.com/NervJS/parse-css-to-stylesheet/commit/38e6856e034b9260a05d34f2c792aa1a9283b85e))



## [0.0.57](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.56...v0.0.57) (2024-04-19)


### Bug Fixes

* 修复export default 没有包裹combine ([38dcd1d](https://github.com/NervJS/parse-css-to-stylesheet/commit/38dcd1dfa1ba5cd3b3492b6e99d678f07ea37d71))



## [0.0.56](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.55...v0.0.56) (2024-04-18)


### Features

* 支持解析hoc ([5f7932c](https://github.com/NervJS/parse-css-to-stylesheet/commit/5f7932cc0306c8d8157055ccdbf8e16d744a1282))



## [0.0.55](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.54...v0.0.55) (2024-04-17)


### Features

* 移除无用配置 ([358cbfa](https://github.com/NervJS/parse-css-to-stylesheet/commit/358cbfa86eeba8a5b83df462dc367d203e0e3611))
* 支持env ([9c5b851](https://github.com/NervJS/parse-css-to-stylesheet/commit/9c5b851c0d51dda52fa9a3d5ded080ab9be9ac44))



## [0.0.54](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.53...v0.0.54) (2024-04-15)



## [0.0.53](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.52...v0.0.53) (2024-04-15)


### Bug Fixes

* 修复rgba精度丢失 ([81c0914](https://github.com/NervJS/parse-css-to-stylesheet/commit/81c091489608939568d02fb1e03f592df148100b))



## [0.0.52](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.51...v0.0.52) (2024-04-11)


### Bug Fixes

* 修复rotate解析异常 ([30dcdeb](https://github.com/NervJS/parse-css-to-stylesheet/commit/30dcdeb2b5cbc2da028a3866ab2ff129f5ebed36))


### Features

* 更新快照 ([fc197b3](https://github.com/NervJS/parse-css-to-stylesheet/commit/fc197b32544bf5545f141f8c166cb23c0eed369d))
* 增加animation、keyframe的解析 ([09fcb0e](https://github.com/NervJS/parse-css-to-stylesheet/commit/09fcb0e4b9360ed7e3ecaa658e64d393b312f422))
* 增加animation的duration计算 ([0bb6a8c](https://github.com/NervJS/parse-css-to-stylesheet/commit/0bb6a8c99012e0140388ac7f0387c4064384c7b3))
* 支持解析animation ([2fe6a2f](https://github.com/NervJS/parse-css-to-stylesheet/commit/2fe6a2fe56975a91a315d76843186c2d73669387))



## [0.0.51](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.50...v0.0.51) (2024-04-09)



## [0.0.50](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.49...v0.0.50) (2024-04-08)


### Features

* 支持overflow:auto ([6983bd5](https://github.com/NervJS/parse-css-to-stylesheet/commit/6983bd5e1d7e8b62e9041ba288440b2b06665488))



## [0.0.49](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.48...v0.0.49) (2024-04-01)


### Bug Fixes

* 修复部分组件没有包裹combine ([adeae60](https://github.com/NervJS/parse-css-to-stylesheet/commit/adeae60a5d97190097fc2c0b4811d6c6321614a5))



## [0.0.48](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.47...v0.0.48) (2024-03-29)


### Features

* 支径向渐变 ([1962ab3](https://github.com/NervJS/parse-css-to-stylesheet/commit/1962ab3f5b8052c451704b5ff8bb86cc36a472f8))



## [0.0.47](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.46...v0.0.47) (2024-03-27)


### Features

* 移除编译时处理style ([e1af9d3](https://github.com/NervJS/parse-css-to-stylesheet/commit/e1af9d34f1e25ef6933a6b2f63a0e97674ecc8d8))



## [0.0.46](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.45...v0.0.46) (2024-03-27)


### Features

* 支持伪类first-child\last-child\nth-child ([ee404ef](https://github.com/NervJS/parse-css-to-stylesheet/commit/ee404ef9f4bf21fe25c2e2cc100b959aecf7c74f))



## [0.0.45](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.44...v0.0.45) (2024-03-26)


### Features

* 增加半编译支持 ([318fa24](https://github.com/NervJS/parse-css-to-stylesheet/commit/318fa244ceecbb57fb7f0100aca7225ec842a9aa))



## [0.0.44](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.43...v0.0.44) (2024-03-26)


### Bug Fixes

* 移除css变量逻辑，迁移到postcss处理 ([24f8a46](https://github.com/NervJS/parse-css-to-stylesheet/commit/24f8a46aae8940e5f117e47e935cf57503f1cca6))



## [0.0.43](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.42...v0.0.43) (2024-03-21)


### Features

* 增加属性测试用力、修改swc版本，统一从swc_core获取 ([57fdfcf](https://github.com/NervJS/parse-css-to-stylesheet/commit/57fdfcf182c3074ec479e6a41aaf627280839340))



## [0.0.42](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.41...v0.0.42) (2024-03-20)


### Features

* 支持css变量 ([680176f](https://github.com/NervJS/parse-css-to-stylesheet/commit/680176fdbf3dc2342d25b3f69abcd544d2f0185d))



## [0.0.41](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.40...v0.0.41) (2024-03-19)


### Features

* 支持解析高阶组件 ([e75aa7f](https://github.com/NervJS/parse-css-to-stylesheet/commit/e75aa7fae41b6280eff7779bfea72d13e309cab6))



## [0.0.40](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.39...v0.0.40) (2024-03-18)


### Features

* 支持跨组件传递class和style ([5d785cf](https://github.com/NervJS/parse-css-to-stylesheet/commit/5d785cf6731d27ae00247963fabf633021983b2d))



## [0.0.39](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.38...v0.0.39) (2024-03-14)


### Features

* 支持calc，支持大小写px ([5c39eb4](https://github.com/NervJS/parse-css-to-stylesheet/commit/5c39eb4b1fec994c6913cf5b31366feaaa57cd29))



## [0.0.38](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.37...v0.0.38) (2024-03-13)


### Features

* 支持箭头函数的解析 ([96166e5](https://github.com/NervJS/parse-css-to-stylesheet/commit/96166e56855935163995b2b72a8f34c4f93fea22))



## [0.0.37](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.36...v0.0.37) (2024-03-13)


### Features

* 替换正则库，删除无用代码，提升性能 ([44efe6b](https://github.com/NervJS/parse-css-to-stylesheet/commit/44efe6b3383264098449c24e97dfef734566af10))



## [0.0.36](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.35...v0.0.36) (2024-03-12)


### Features

* 修改文档 ([6987591](https://github.com/NervJS/parse-css-to-stylesheet/commit/6987591858e0e9b847b109fb37b4e0065afc7564))
* 支持多类选择器、修复rgba等小问题 ([0f3648b](https://github.com/NervJS/parse-css-to-stylesheet/commit/0f3648b81cb1e895681c9131c104b2cd1cd95382))



## [0.0.35](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.34...v0.0.35) (2024-03-07)


### Bug Fixes

* 当返回值为 JSX 时才处理层叠 ([d2bbcc8](https://github.com/NervJS/parse-css-to-stylesheet/commit/d2bbcc84b6a07a1feabc3720f9e4d8cb8255710f))
* 修复嵌套和伪类的bug ([feba800](https://github.com/NervJS/parse-css-to-stylesheet/commit/feba800e7cab86c3b99241b8a844c67c59503905))



## [0.0.34](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.33...v0.0.34) (2024-03-06)


### Features

* 给组件返回值包裹用于层叠的函数 ([aa0c128](https://github.com/NervJS/parse-css-to-stylesheet/commit/aa0c128d1b21d1ceff084ee471cd91dfa4dc7939))
* 完善组件返回值包裹用于层叠的函数 ([4c15e38](https://github.com/NervJS/parse-css-to-stylesheet/commit/4c15e380fe2ef805c485a7313c84fbbc031e16e0))
* 新增paser配置 ([fca94e6](https://github.com/NervJS/parse-css-to-stylesheet/commit/fca94e65f6e92e1bcf54f37f84fd2a709590d25a))
* 增加嵌套拆分逻辑 ([4af4b9a](https://github.com/NervJS/parse-css-to-stylesheet/commit/4af4b9ad0509b815041141cdb943b61eca217d18))
* 增加权重逻辑 ([a5bb785](https://github.com/NervJS/parse-css-to-stylesheet/commit/a5bb7859dcd15b1fd2dd0bc5df4005adb5fbaaea))



## [0.0.33](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.32...v0.0.33) (2024-03-05)


### Bug Fixes

* 修复border报错 ([6f107df](https://github.com/NervJS/parse-css-to-stylesheet/commit/6f107df252cacde364e549ee20c50cde2e1a208f))



## [0.0.32](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.31...v0.0.32) (2024-03-05)


### Features

* 补充background相关属性 ([66d8225](https://github.com/NervJS/parse-css-to-stylesheet/commit/66d8225d9d17e1dc492c74a8544b55765b0841e8))
* 打通rn的module接入流程及补充部分样式的解析 ([43ab51e](https://github.com/NervJS/parse-css-to-stylesheet/commit/43ab51ee13dd4a4d00de8f4c1885afe93dc99b1f))
* 添加单独编译rn的css的入口、增加rn测试案例场景 ([5a0c11f](https://github.com/NervJS/parse-css-to-stylesheet/commit/5a0c11fe0047430bb745dc1f14a2c61682040824))
* 完成布局属性的处理 ([a276442](https://github.com/NervJS/parse-css-to-stylesheet/commit/a276442ed458504f80b8516cd9093fd50cd51311))
* 完成所有文本样式的解析 ([a7a9e18](https://github.com/NervJS/parse-css-to-stylesheet/commit/a7a9e18a80bf2a9d2a0ded402bd7b2c7750f9dcd))
* 完成完整的重构 ([42a42a8](https://github.com/NervJS/parse-css-to-stylesheet/commit/42a42a8c0acf51cadd76d07fec452b5ce5a24694))
* 完成转换工作 ([12a65e0](https://github.com/NervJS/parse-css-to-stylesheet/commit/12a65e0c49c1f520a4d255758939862f24a2c495))
* 完成jsx和css的融合对接 ([5021e29](https://github.com/NervJS/parse-css-to-stylesheet/commit/5021e29714cbdb5df241473f9609c53706da2f2f))
* 完善transform、margin、padding、flex ([2226548](https://github.com/NervJS/parse-css-to-stylesheet/commit/222654854af0dac7c7007b9c1dfa92752eac6d41))
* 增加对border的支持 ([71f8f1f](https://github.com/NervJS/parse-css-to-stylesheet/commit/71f8f1f3ef679ff45fc65713ef69efdfbc42cdad))
* 增加属性一对多的输出逻辑、增加部分文本类样式解析 ([78f53f5](https://github.com/NervJS/parse-css-to-stylesheet/commit/78f53f5731a385d5f8f21123077ccf089ec66f86))
* 增加伪类 ([0ed8cb3](https://github.com/NervJS/parse-css-to-stylesheet/commit/0ed8cb3cb64952f1ca9270d4dfdf41868a459241))
* 增加transformOrigin ([35369db](https://github.com/NervJS/parse-css-to-stylesheet/commit/35369db8839d68702066dd6b0ee845cfd5854fda))
* 增加transformOrigin ([5aa0b05](https://github.com/NervJS/parse-css-to-stylesheet/commit/5aa0b058c492a7437a3d7e83fa85a9cfbb7ea040))



## [0.0.31](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.30...v0.0.31) (2024-02-23)


### Bug Fixes

* 修复条件判断不正确导致的静态样式被转换成动态样式的问题，并更新项目测试快照 ([151b5c3](https://github.com/NervJS/parse-css-to-stylesheet/commit/151b5c3f44464dccf48978f435bf22482ad270f4))
* 修复以前样式解析写法不严谨导致的报错问题以及 ets 样式名错误问题 ([cf6e958](https://github.com/NervJS/parse-css-to-stylesheet/commit/cf6e958c2806d1e0e3a4eeed9129c6cf373211eb))
* 修复letterspacing的名字错误 ([92363f0](https://github.com/NervJS/parse-css-to-stylesheet/commit/92363f00060b2138d750cb97988eb0d1fcb6c35b))
* 修复px数值单位匹配丢失了负数和小数匹配错误问题 ([8981589](https://github.com/NervJS/parse-css-to-stylesheet/commit/8981589cec79da20c3aed51e55869adec84f1259))


### Features

* 扁平化样式解析 ([c5b68c9](https://github.com/NervJS/parse-css-to-stylesheet/commit/c5b68c9b1832bf181cc41b7cce5858bbe1fa41e7))
* 抽离写入逻辑的公共函数，支持 React.createElement 形式的代码 ([d2731f2](https://github.com/NervJS/parse-css-to-stylesheet/commit/d2731f268651ce76f42caa993b893b9da9ec984e))
* 接入 React.createElement 的输入解析 ([29041fe](https://github.com/NervJS/parse-css-to-stylesheet/commit/29041fe4bea8c41dae2c53859eba1e56572ad154))
* 让dynmaic计算放到运行时获取 ([7ba90f8](https://github.com/NervJS/parse-css-to-stylesheet/commit/7ba90f84ef95dd2ac3648e3d5498669a245cb4ab))
* 修改测试案例 ([670bab9](https://github.com/NervJS/parse-css-to-stylesheet/commit/670bab9d9203d151acd5cd7d13a4f604e9ba1a24))
* 修改引入包的名字 ([6371b32](https://github.com/NervJS/parse-css-to-stylesheet/commit/6371b32cab3824cb1bfb26f3e7f0e4e1d45ec553))
* 移除对taro-component的标签判断识别 ([36c8a3f](https://github.com/NervJS/parse-css-to-stylesheet/commit/36c8a3f194f3517fbe5df2cee5c68d970e055031))
* 增加LineHeight\LineSpacing\TextAlign\TextOverflow\FontWeight解析 ([4ebf7b5](https://github.com/NervJS/parse-css-to-stylesheet/commit/4ebf7b5fc6ceaa1cad6bbc65d9c8224f1aa495b8))
* 支持rgba、支持font-style解析、border解析、补充部分遗漏属性 ([1c74b9f](https://github.com/NervJS/parse-css-to-stylesheet/commit/1c74b9fa7b60fc602b1f9e5f345ed09feeabe11f))
* 支持vw\vh ([4f56f05](https://github.com/NervJS/parse-css-to-stylesheet/commit/4f56f054de94b1992a797e76dfe90be500b0bfa6))
* px单位单独处理转换 ([d23b839](https://github.com/NervJS/parse-css-to-stylesheet/commit/d23b839b7b9f9c22b3d7fe76c590742dff7b0dc3))



## [0.0.22](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.21...v0.0.22) (2023-11-06)


### Features

* 更新测试用例 ([d999184](https://github.com/NervJS/parse-css-to-stylesheet/commit/d99918498db29795df0f18ef6e1caa45e743955d))
* 将不处理的属性进行移除 ([7668be8](https://github.com/NervJS/parse-css-to-stylesheet/commit/7668be8f20e06eeac2cfa2852dc4a4d5d98d210e))



## [0.0.21](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.20...v0.0.21) (2023-11-02)


### Features

* 完善代码 ([7fe9036](https://github.com/NervJS/parse-css-to-stylesheet/commit/7fe9036a60cb84db5526b191958417bad2a4579e))
* 优化内容、增加对border、constraintSize的解析、优化margin、padding逻辑；增加属性名的标记 ([71effc7](https://github.com/NervJS/parse-css-to-stylesheet/commit/71effc712c221efc956ec354b5666195f33c79d3))



## [0.0.20](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.19...v0.0.20) (2023-10-30)


### Bug Fixes

* 修复逻辑错误 ([768e437](https://github.com/NervJS/parse-css-to-stylesheet/commit/768e4379b78a677f2bbd082fde51d02589599cca))



## [0.0.19](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.18...v0.0.19) (2023-10-27)


### Bug Fixes

* 修复 align-items 丢失问题 ([cc4618e](https://github.com/NervJS/parse-css-to-stylesheet/commit/cc4618e7c3d6fcb439f966329d834735a3214e99))
* 修复动态样式，类名静态时，调用 calcDynamicStyle 函数第二参数为 null ([d5f6091](https://github.com/NervJS/parse-css-to-stylesheet/commit/d5f6091bd7e7916ffc74099133b02c0b7b4d4c3b))
* 修复对 linearGradient 的角度及 color-stop 处理 ([22f648a](https://github.com/NervJS/parse-css-to-stylesheet/commit/22f648ac9e3a7578b3a7462f17819b5da77cc6d3))
* 修复样式单独设置 background 的 color 值时丢失 background color 的问题 ([6103a4c](https://github.com/NervJS/parse-css-to-stylesheet/commit/6103a4c2c04161d69ecb107306384b529578c535))
* margin/padding 为 0 被忽略 ([b9eeec0](https://github.com/NervJS/parse-css-to-stylesheet/commit/b9eeec0271e514f4480c0f6197ca8c551c72dd23))



## [0.0.18](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.17...v0.0.18) (2023-10-27)


### Features

* 将样式转换处理拆分出去 ([6de1b02](https://github.com/NervJS/parse-css-to-stylesheet/commit/6de1b029b06a45e2dd5298e4ca883425b5d438c6))



## [0.0.17](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.16...v0.0.17) (2023-10-25)


### Bug Fixes

* 优化 Background 处理 ([c4a4367](https://github.com/NervJS/parse-css-to-stylesheet/commit/c4a4367aed0c29651e607acea6cc7728d2040cde))


### Features

* 调整生成逻辑 ([84dcb1a](https://github.com/NervJS/parse-css-to-stylesheet/commit/84dcb1ad9984b3446a5dcc2aa81c6bfcf6dc7252))
* 支持解析 css 代码中的 transform 属性 ([197a922](https://github.com/NervJS/parse-css-to-stylesheet/commit/197a92285d055ee51fca38888e523275ec9c3d71))



## [0.0.16](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.14...v0.0.16) (2023-10-24)


### Bug Fixes

* 当用到 __inner_style__ 时才插入 ([c10cffc](https://github.com/NervJS/parse-css-to-stylesheet/commit/c10cffc7093b9e2878ee59fdd1038ec774934713))


### Features

* 不拆分 Background ([0f3111b](https://github.com/NervJS/parse-css-to-stylesheet/commit/0f3111b023ca91095d2fdaf55e4d0388a626b49a))
* 将线性渐变解析出来 ([3d8ddb6](https://github.com/NervJS/parse-css-to-stylesheet/commit/3d8ddb6ede437c2fbc9c35e938dfd87204979201))
* 优化样式解析处理 ([efe85d3](https://github.com/NervJS/parse-css-to-stylesheet/commit/efe85d3bc41986fb12dceb4ccb361eba5f53bc11))
* 支持 background 相关属性解析 ([7fdd888](https://github.com/NervJS/parse-css-to-stylesheet/commit/7fdd888776f93ecc99dd5a43e2d90b4d5b3921cc))
* 支持 flex 相关属性解析转换 ([f2caa6b](https://github.com/NervJS/parse-css-to-stylesheet/commit/f2caa6b8be2b94722139637c58d3c54a63ed7255))
* 支持拆解处理 style 属性中的渐变 ([e7f9391](https://github.com/NervJS/parse-css-to-stylesheet/commit/e7f9391a801cec327e3343b05d914d0593b761d0))
* style 属性中也支持写 Background 相关样式属性 ([91f611f](https://github.com/NervJS/parse-css-to-stylesheet/commit/91f611f33340dc394ef312abc9db69ad5583354e))



## [0.0.14](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.13...v0.0.14) (2023-10-19)


### Bug Fixes

* TextDecoration => TextDecorationType ([c9167e4](https://github.com/NervJS/parse-css-to-stylesheet/commit/c9167e431a5ad8abece2319519d4f225a0366035))



## [0.0.13](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.11...v0.0.13) (2023-10-19)


### Bug Fixes

* 遗漏样式属性添加情况 ([b698459](https://github.com/NervJS/parse-css-to-stylesheet/commit/b698459608d91030b2c4b8ba28c46e15bbe08189))


### Features

* 开始尝试在在编译时按需处理样式 ([82aa6f5](https://github.com/NervJS/parse-css-to-stylesheet/commit/82aa6f5a3a715d7e3ac50ac784b3a19cd4fe06e6))
* 优化样式处理逻辑 ([d8954ae](https://github.com/NervJS/parse-css-to-stylesheet/commit/d8954aed25438151a7c48e477b462dd0235507d5))
* 支持 margin/padding/borderRadius 各自的 longhand 样式名 ([7e112f6](https://github.com/NervJS/parse-css-to-stylesheet/commit/7e112f6dff71e35a289bab646c993f1144bfd9fe))
* 支持 style 属性中写 lognhand 样式 ([68246b2](https://github.com/NervJS/parse-css-to-stylesheet/commit/68246b2757a9fc6ee468c937962920e50ca1a932))
* 支持优化处理 border-radius 属性 ([8a79d76](https://github.com/NervJS/parse-css-to-stylesheet/commit/8a79d7660cdac449f214bb79d9d25ff041aa70e4))
* 支持优化处理 text-decoration 属性 ([9799f94](https://github.com/NervJS/parse-css-to-stylesheet/commit/9799f94e9136346e52e2029099b991bfb077f643))



## [0.0.11](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.10...v0.0.11) (2023-10-17)


### Features

* 将处理动态类的方法踢出去 ([f767717](https://github.com/NervJS/parse-css-to-stylesheet/commit/f767717ab9f9b358afe150d8f56689bbcdc586f3))



## [0.0.10](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.9...v0.0.10) (2023-10-17)


### Bug Fixes

* __inner_style__ 要符合 JSON 定义 ([94a533e](https://github.com/NervJS/parse-css-to-stylesheet/commit/94a533eff0531453f8932a74e3672a3d16fe934c))



## [0.0.9](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.8...v0.0.9) (2023-10-17)


### Bug Fixes

* 避免将 rgab 色值转为 hexalpha ([ccbe771](https://github.com/NervJS/parse-css-to-stylesheet/commit/ccbe771da730911712df300cddf4479437e5eb80))
* 将样式名转为 camelCase ([cc39775](https://github.com/NervJS/parse-css-to-stylesheet/commit/cc3977548b3a0fb898cc499b6a2d493d19d1a491))



## [0.0.8](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.7...v0.0.8) (2023-10-17)


### Bug Fixes

* 去掉样式继承逻辑 ([eec198f](https://github.com/NervJS/parse-css-to-stylesheet/commit/eec198f20bd0a93e163c5990424a19322ca9a94a))
* 去掉样式继承逻辑，仅支持类选择器 ([0752a13](https://github.com/NervJS/parse-css-to-stylesheet/commit/0752a13770e2387b3b6479cca7472783fe12f3bb))


### Features

* 将 __calc_style__ 函数插入到代码中 ([65ec163](https://github.com/NervJS/parse-css-to-stylesheet/commit/65ec1632f9031eb346fb90993031b1f977237170))
* 将样式转为对象插入到代码中 ([644a0d3](https://github.com/NervJS/parse-css-to-stylesheet/commit/644a0d3018efb34c0a9c1afda918034a6f13eb05))
* 支持处理动态类名情况 ([ae91b2c](https://github.com/NervJS/parse-css-to-stylesheet/commit/ae91b2c31362ff1d073e3bc268d3aa43fac7cb51))



## [0.0.7](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.6...v0.0.7) (2023-10-12)


### Bug Fixes

* 修复样式继承 ([771ffd5](https://github.com/NervJS/parse-css-to-stylesheet/commit/771ffd51b9ec65097cbc2ef78a7a3b650e6e6d04))



## [0.0.6](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.5...v0.0.6) (2023-10-12)


### Bug Fixes

* 暂时取消支持样式继承 ([dfeef03](https://github.com/NervJS/parse-css-to-stylesheet/commit/dfeef035abda6dd4e4f94e03e7752a57f2cbe862))



## [0.0.5](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.4...v0.0.5) (2023-10-11)


### Bug Fixes

* 修复 jsx 遍历不完整问题 ([cec4e20](https://github.com/NervJS/parse-css-to-stylesheet/commit/cec4e20710ec6f621cdff93961e5df3897999dd1))


### Features

* 去掉 typescript 声明 ([3c68159](https://github.com/NervJS/parse-css-to-stylesheet/commit/3c68159872f082f55081a978d4da5bc8805abf69))



## [0.0.4](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.3...v0.0.4) (2023-10-11)


### Features

* 支持为 taro 组件库引入组件增加样式 ([6eb025f](https://github.com/NervJS/parse-css-to-stylesheet/commit/6eb025f1024d09091e065baa69efe4dbe1c601e8))



## [0.0.3](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.2...v0.0.3) (2023-10-11)


### Bug Fixes

* 修复 JSX 解析报错问题 ([7818a16](https://github.com/NervJS/parse-css-to-stylesheet/commit/7818a163712adada628ec3ece35c2a7c18a7b46d))



## [0.0.2](https://github.com/NervJS/parse-css-to-stylesheet/compare/710d95c199e15641bf05e04bea0b6ca5965f4bca...v0.0.2) (2023-10-10)


### Features

* 处理不可继承的样式的情况，同时支持样式值为 inherit 的情况 ([13ae716](https://github.com/NervJS/parse-css-to-stylesheet/commit/13ae7167d4cbeb6fc880d764301320909530f236))
* 代码解析支持装饰器 ([c5d8522](https://github.com/NervJS/parse-css-to-stylesheet/commit/c5d852215a8f98fa5137e691b9343232280a1587))
* 调整代码 ([9c5f5d7](https://github.com/NervJS/parse-css-to-stylesheet/commit/9c5f5d7b2f1269b39b9a90f446f6262c16c93d6d))
* 根据选择器特异性排序计算出每个节点的最终样式 ([070ce57](https://github.com/NervJS/parse-css-to-stylesheet/commit/070ce57bf24a197e614a2885913818e46ec4be50))
* 基于 napi-rs 改造 ([19a6f31](https://github.com/NervJS/parse-css-to-stylesheet/commit/19a6f3168a5e9719962ab389266af27cb2192aca))
* 计算出每一个节点的所有样式规则 ([0082034](https://github.com/NervJS/parse-css-to-stylesheet/commit/008203446135c8c6dbc7795bce957a8826d0bb47))
* 将 ast 中的 jsx tree 转为 ego tree ([34a1be5](https://github.com/NervJS/parse-css-to-stylesheet/commit/34a1be58c9466c6f6b1a141efbd225268d10ce89))
* 解析 JSX 构建 ego tree ([949a1ec](https://github.com/NervJS/parse-css-to-stylesheet/commit/949a1ecf1fc9f660e998db8e68be374786e1717a))
* 实现根据选择器查找 rust 文本节点，同时梳理代码 ([e850496](https://github.com/NervJS/parse-css-to-stylesheet/commit/e8504965842280b86a9f23d3b0c6040bf6123343))
* 实现样式继承 ([d508d15](https://github.com/NervJS/parse-css-to-stylesheet/commit/d508d1569878ec41abb1ca2d1567c39a8108258b))
* 增加 ElementRef ([710d95c](https://github.com/NervJS/parse-css-to-stylesheet/commit/710d95c199e15641bf05e04bea0b6ca5965f4bca))
* 增加 style parser ([a4e3f11](https://github.com/NervJS/parse-css-to-stylesheet/commit/a4e3f11eb9bc3e909c616cee03865304e304c9f1))
* 增加对 JSX 子树的支持 ([c73bbe5](https://github.com/NervJS/parse-css-to-stylesheet/commit/c73bbe5e4f9234fed2b582de1f1883dc84cd0d3d))
* 增加记录每段样式的特异性 ([7c6b9c3](https://github.com/NervJS/parse-css-to-stylesheet/commit/7c6b9c3c49109fc1dbcf6a63ce54839bec748c73))
* 支持 JSX 节点没有 style 属性情况下，将节点对应的样式写入到 style 属性中 ([d9eb803](https://github.com/NervJS/parse-css-to-stylesheet/commit/d9eb803259e895c8332e379aae0209e184f37a19))
* 支持 React.Fragment 用法 ([ac404c8](https://github.com/NervJS/parse-css-to-stylesheet/commit/ac404c8d69f9a0ca252fb9c12b5b0d3ba6e410a9))
* 支持函数调用方式的子 JSX 拆分写法 ([73a5bcb](https://github.com/NervJS/parse-css-to-stylesheet/commit/73a5bcbf34bea9e3301ab0f3d88fa9f1de561ad2))
* 支持将样式节点记录写入 ast 中 ([c35cbdf](https://github.com/NervJS/parse-css-to-stylesheet/commit/c35cbdf15e02a773cc912b1eedeaf29922225ac4))
* 支持将样式文件中样式与 JSX 节点的 style 属性值合并 ([1beb45a](https://github.com/NervJS/parse-css-to-stylesheet/commit/1beb45a114c2a649bd46417afe0efb8d74b85f91))
* 支持类组件 ([416e595](https://github.com/NervJS/parse-css-to-stylesheet/commit/416e59575c1ad6c58f19e2cd6f34cec3b38db436))
* update ([ca39a94](https://github.com/NervJS/parse-css-to-stylesheet/commit/ca39a94e3da22b737b079cf34e9383591313519c))



