## [0.0.29](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.28...v0.0.29) (2023-12-20)



## [0.0.28](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.26...v0.0.28) (2023-12-20)


### Features

* 支持rgba、支持font-style解析、border解析、补充部分遗漏属性 ([1c74b9f](https://github.com/NervJS/parse-css-to-stylesheet/commit/1c74b9fa7b60fc602b1f9e5f345ed09feeabe11f))
* 支持vw\vh ([4f56f05](https://github.com/NervJS/parse-css-to-stylesheet/commit/4f56f054de94b1992a797e76dfe90be500b0bfa6))



## [0.0.27](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.26...v0.0.27) (2023-12-19)


### Features

* 支持vw\vh ([4f56f05](https://github.com/NervJS/parse-css-to-stylesheet/commit/4f56f054de94b1992a797e76dfe90be500b0bfa6))



## [0.0.26](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.25...v0.0.26) (2023-11-30)


### Bug Fixes

* 修复以前样式解析写法不严谨导致的报错问题以及 ets 样式名错误问题 ([cf6e958](https://github.com/NervJS/parse-css-to-stylesheet/commit/cf6e958c2806d1e0e3a4eeed9129c6cf373211eb))
* 修复条件判断不正确导致的静态样式被转换成动态样式的问题，并更新项目测试快照 ([151b5c3](https://github.com/NervJS/parse-css-to-stylesheet/commit/151b5c3f44464dccf48978f435bf22482ad270f4))


### Features

* 抽离写入逻辑的公共函数，支持 React.createElement 形式的代码 ([d2731f2](https://github.com/NervJS/parse-css-to-stylesheet/commit/d2731f268651ce76f42caa993b893b9da9ec984e))
* 接入 React.createElement 的输入解析 ([29041fe](https://github.com/NervJS/parse-css-to-stylesheet/commit/29041fe4bea8c41dae2c53859eba1e56572ad154))



## [0.0.25](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.24...v0.0.25) (2023-11-21)


### Features

* 让dynmaic计算放到运行时获取 ([7ba90f8](https://github.com/NervJS/parse-css-to-stylesheet/commit/7ba90f84ef95dd2ac3648e3d5498669a245cb4ab))



## [0.0.24](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.23...v0.0.24) (2023-11-17)


### Bug Fixes

* 修复px数值单位匹配丢失了负数和小数匹配错误问题 ([8981589](https://github.com/NervJS/parse-css-to-stylesheet/commit/8981589cec79da20c3aed51e55869adec84f1259))



## [0.0.23](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.22...v0.0.23) (2023-11-13)

### Bug Fixes

- 修复 letterspacing 的名字错误 ([92363f0](https://github.com/NervJS/parse-css-to-stylesheet/commit/92363f00060b2138d750cb97988eb0d1fcb6c35b))

### Features

- 修改测试案例 ([670bab9](https://github.com/NervJS/parse-css-to-stylesheet/commit/670bab9d9203d151acd5cd7d13a4f604e9ba1a24))
- 修改引入包的名字 ([6371b32](https://github.com/NervJS/parse-css-to-stylesheet/commit/6371b32cab3824cb1bfb26f3e7f0e4e1d45ec553))
- 增加 LineHeight\LineSpacing\TextAlign\TextOverflow\FontWeight 解析 ([4ebf7b5](https://github.com/NervJS/parse-css-to-stylesheet/commit/4ebf7b5fc6ceaa1cad6bbc65d9c8224f1aa495b8))
- px 单位单独处理转换 ([d23b839](https://github.com/NervJS/parse-css-to-stylesheet/commit/d23b839b7b9f9c22b3d7fe76c590742dff7b0dc3))

## [0.0.22](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.21...v0.0.22) (2023-11-06)

### Features

- 更新测试用例 ([d999184](https://github.com/NervJS/parse-css-to-stylesheet/commit/d99918498db29795df0f18ef6e1caa45e743955d))
- 将不处理的属性进行移除 ([7668be8](https://github.com/NervJS/parse-css-to-stylesheet/commit/7668be8f20e06eeac2cfa2852dc4a4d5d98d210e))

## [0.0.21](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.20...v0.0.21) (2023-11-02)

### Features

- 完善代码 ([7fe9036](https://github.com/NervJS/parse-css-to-stylesheet/commit/7fe9036a60cb84db5526b191958417bad2a4579e))
- 优化内容、增加对 border、constraintSize 的解析、优化 margin、padding 逻辑；增加属性名的标记 ([71effc7](https://github.com/NervJS/parse-css-to-stylesheet/commit/71effc712c221efc956ec354b5666195f33c79d3))

## [0.0.20](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.19...v0.0.20) (2023-10-30)

### Bug Fixes

- 修复逻辑错误 ([768e437](https://github.com/NervJS/parse-css-to-stylesheet/commit/768e4379b78a677f2bbd082fde51d02589599cca))

## [0.0.19](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.17...v0.0.19) (2023-10-27)

### Bug Fixes

- 修复 align-items 丢失问题 ([cc4618e](https://github.com/NervJS/parse-css-to-stylesheet/commit/cc4618e7c3d6fcb439f966329d834735a3214e99))
- 修复动态样式，类名静态时，调用 calcDynamicStyle 函数第二参数为 null ([d5f6091](https://github.com/NervJS/parse-css-to-stylesheet/commit/d5f6091bd7e7916ffc74099133b02c0b7b4d4c3b))
- 修复对 linearGradient 的角度及 color-stop 处理 ([22f648a](https://github.com/NervJS/parse-css-to-stylesheet/commit/22f648ac9e3a7578b3a7462f17819b5da77cc6d3))
- 修复样式单独设置 background 的 color 值时丢失 background color 的问题 ([6103a4c](https://github.com/NervJS/parse-css-to-stylesheet/commit/6103a4c2c04161d69ecb107306384b529578c535))
- margin/padding 为 0 被忽略 ([b9eeec0](https://github.com/NervJS/parse-css-to-stylesheet/commit/b9eeec0271e514f4480c0f6197ca8c551c72dd23))

### Features

- 将样式转换处理拆分出去 ([6de1b02](https://github.com/NervJS/parse-css-to-stylesheet/commit/6de1b029b06a45e2dd5298e4ca883425b5d438c6))

## [0.0.18](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.17...v0.0.18) (2023-10-26)

### Features

- 将样式转换处理拆分出去 ([6de1b02](https://github.com/NervJS/parse-css-to-stylesheet/commit/6de1b029b06a45e2dd5298e4ca883425b5d438c6))

## [0.0.17](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.15...v0.0.17) (2023-10-25)

### Bug Fixes

- 当用到 **inner_style** 时才插入 ([c10cffc](https://github.com/NervJS/parse-css-to-stylesheet/commit/c10cffc7093b9e2878ee59fdd1038ec774934713))
- 优化 Background 处理 ([c4a4367](https://github.com/NervJS/parse-css-to-stylesheet/commit/c4a4367aed0c29651e607acea6cc7728d2040cde))

### Features

- 不拆分 Background ([0f3111b](https://github.com/NervJS/parse-css-to-stylesheet/commit/0f3111b023ca91095d2fdaf55e4d0388a626b49a))
- 调整生成逻辑 ([84dcb1a](https://github.com/NervJS/parse-css-to-stylesheet/commit/84dcb1ad9984b3446a5dcc2aa81c6bfcf6dc7252))
- 支持解析 css 代码中的 transform 属性 ([197a922](https://github.com/NervJS/parse-css-to-stylesheet/commit/197a92285d055ee51fca38888e523275ec9c3d71))

## [0.0.16](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.15...v0.0.16) (2023-10-24)

### Bug Fixes

- 当用到 **inner_style** 时才插入 ([c10cffc](https://github.com/NervJS/parse-css-to-stylesheet/commit/c10cffc7093b9e2878ee59fdd1038ec774934713))

### Features

- 不拆分 Background ([0f3111b](https://github.com/NervJS/parse-css-to-stylesheet/commit/0f3111b023ca91095d2fdaf55e4d0388a626b49a))

## [0.0.15](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.14...v0.0.15) (2023-10-24)

### Features

- 将线性渐变解析出来 ([3d8ddb6](https://github.com/NervJS/parse-css-to-stylesheet/commit/3d8ddb6ede437c2fbc9c35e938dfd87204979201))
- 优化样式解析处理 ([efe85d3](https://github.com/NervJS/parse-css-to-stylesheet/commit/efe85d3bc41986fb12dceb4ccb361eba5f53bc11))
- 支持 background 相关属性解析 ([7fdd888](https://github.com/NervJS/parse-css-to-stylesheet/commit/7fdd888776f93ecc99dd5a43e2d90b4d5b3921cc))
- 支持 flex 相关属性解析转换 ([f2caa6b](https://github.com/NervJS/parse-css-to-stylesheet/commit/f2caa6b8be2b94722139637c58d3c54a63ed7255))
- 支持拆解处理 style 属性中的渐变 ([e7f9391](https://github.com/NervJS/parse-css-to-stylesheet/commit/e7f9391a801cec327e3343b05d914d0593b761d0))
- style 属性中也支持写 Background 相关样式属性 ([91f611f](https://github.com/NervJS/parse-css-to-stylesheet/commit/91f611f33340dc394ef312abc9db69ad5583354e))

## [0.0.14](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.13...v0.0.14) (2023-10-19)

### Bug Fixes

- TextDecoration => TextDecorationType ([c9167e4](https://github.com/NervJS/parse-css-to-stylesheet/commit/c9167e431a5ad8abece2319519d4f225a0366035))

## [0.0.13](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.11...v0.0.13) (2023-10-19)

### Bug Fixes

- 遗漏样式属性添加情况 ([b698459](https://github.com/NervJS/parse-css-to-stylesheet/commit/b698459608d91030b2c4b8ba28c46e15bbe08189))

### Features

- 开始尝试在在编译时按需处理样式 ([82aa6f5](https://github.com/NervJS/parse-css-to-stylesheet/commit/82aa6f5a3a715d7e3ac50ac784b3a19cd4fe06e6))
- 优化样式处理逻辑 ([d8954ae](https://github.com/NervJS/parse-css-to-stylesheet/commit/d8954aed25438151a7c48e477b462dd0235507d5))
- 支持 margin/padding/borderRadius 各自的 longhand 样式名 ([7e112f6](https://github.com/NervJS/parse-css-to-stylesheet/commit/7e112f6dff71e35a289bab646c993f1144bfd9fe))
- 支持 style 属性中写 lognhand 样式 ([68246b2](https://github.com/NervJS/parse-css-to-stylesheet/commit/68246b2757a9fc6ee468c937962920e50ca1a932))
- 支持优化处理 border-radius 属性 ([8a79d76](https://github.com/NervJS/parse-css-to-stylesheet/commit/8a79d7660cdac449f214bb79d9d25ff041aa70e4))
- 支持优化处理 text-decoration 属性 ([9799f94](https://github.com/NervJS/parse-css-to-stylesheet/commit/9799f94e9136346e52e2029099b991bfb077f643))

## [0.0.12](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.11...v0.0.12) (2023-10-18)

### Bug Fixes

- 遗漏样式属性添加情况 ([b698459](https://github.com/NervJS/parse-css-to-stylesheet/commit/b698459608d91030b2c4b8ba28c46e15bbe08189))

### Features

- 开始尝试在在编译时按需处理样式 ([82aa6f5](https://github.com/NervJS/parse-css-to-stylesheet/commit/82aa6f5a3a715d7e3ac50ac784b3a19cd4fe06e6))
- 优化样式处理逻辑 ([d8954ae](https://github.com/NervJS/parse-css-to-stylesheet/commit/d8954aed25438151a7c48e477b462dd0235507d5))
- 支持优化处理 text-decoration 属性 ([9799f94](https://github.com/NervJS/parse-css-to-stylesheet/commit/9799f94e9136346e52e2029099b991bfb077f643))

## [0.0.11](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.10...v0.0.11) (2023-10-17)

## [0.0.10](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.8...v0.0.10) (2023-10-17)

### Bug Fixes

- **inner_style** 要符合 JSON 定义 ([94a533e](https://github.com/NervJS/parse-css-to-stylesheet/commit/94a533eff0531453f8932a74e3672a3d16fe934c))
- 避免将 rgab 色值转为 hexalpha ([ccbe771](https://github.com/NervJS/parse-css-to-stylesheet/commit/ccbe771da730911712df300cddf4479437e5eb80))
- 将样式名转为 camelCase ([cc39775](https://github.com/NervJS/parse-css-to-stylesheet/commit/cc3977548b3a0fb898cc499b6a2d493d19d1a491))

## [0.0.9](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.8...v0.0.9) (2023-10-17)

### Bug Fixes

- 避免将 rgab 色值转为 hexalpha ([ccbe771](https://github.com/NervJS/parse-css-to-stylesheet/commit/ccbe771da730911712df300cddf4479437e5eb80))
- 将样式名转为 camelCase ([cc39775](https://github.com/NervJS/parse-css-to-stylesheet/commit/cc3977548b3a0fb898cc499b6a2d493d19d1a491))

## [0.0.8](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.7...v0.0.8) (2023-10-17)

### Bug Fixes

- 去掉样式继承逻辑 ([eec198f](https://github.com/NervJS/parse-css-to-stylesheet/commit/eec198f20bd0a93e163c5990424a19322ca9a94a))
- 去掉样式继承逻辑，仅支持类选择器 ([0752a13](https://github.com/NervJS/parse-css-to-stylesheet/commit/0752a13770e2387b3b6479cca7472783fe12f3bb))

### Features

- 将 **calc_style** 函数插入到代码中 ([65ec163](https://github.com/NervJS/parse-css-to-stylesheet/commit/65ec1632f9031eb346fb90993031b1f977237170))
- 将样式转为对象插入到代码中 ([644a0d3](https://github.com/NervJS/parse-css-to-stylesheet/commit/644a0d3018efb34c0a9c1afda918034a6f13eb05))
- 支持处理动态类名情况 ([ae91b2c](https://github.com/NervJS/parse-css-to-stylesheet/commit/ae91b2c31362ff1d073e3bc268d3aa43fac7cb51))

## [0.0.7](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.6...v0.0.7) (2023-10-12)

### Bug Fixes

- 修复样式继承 ([771ffd5](https://github.com/NervJS/parse-css-to-stylesheet/commit/771ffd51b9ec65097cbc2ef78a7a3b650e6e6d04))

## [0.0.6](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.5...v0.0.6) (2023-10-12)

## [0.0.5](https://github.com/NervJS/parse-css-to-stylesheet/compare/0.0.4...0.0.5) (2023-10-11)

### Bug Fixes

- 修复 jsx 遍历不完整问题 ([cec4e20](https://github.com/NervJS/parse-css-to-stylesheet/commit/cec4e20710ec6f621cdff93961e5df3897999dd1))

### Features

- 去掉 typescript 声明 ([3c68159](https://github.com/NervJS/parse-css-to-stylesheet/commit/3c68159872f082f55081a978d4da5bc8805abf69))

## 0.0.4 (2023-10-11)

### Bug Fixes

- 修复 JSX 解析报错问题 ([7818a16](https://github.com/NervJS/parse-css-to-stylesheet/commit/7818a163712adada628ec3ece35c2a7c18a7b46d))

### Features

- 处理不可继承的样式的情况，同时支持样式值为 inherit 的情况 ([13ae716](https://github.com/NervJS/parse-css-to-stylesheet/commit/13ae7167d4cbeb6fc880d764301320909530f236))
- 代码解析支持装饰器 ([c5d8522](https://github.com/NervJS/parse-css-to-stylesheet/commit/c5d852215a8f98fa5137e691b9343232280a1587))
- 调整代码 ([9c5f5d7](https://github.com/NervJS/parse-css-to-stylesheet/commit/9c5f5d7b2f1269b39b9a90f446f6262c16c93d6d))
- 根据选择器特异性排序计算出每个节点的最终样式 ([070ce57](https://github.com/NervJS/parse-css-to-stylesheet/commit/070ce57bf24a197e614a2885913818e46ec4be50))
- 基于 napi-rs 改造 ([19a6f31](https://github.com/NervJS/parse-css-to-stylesheet/commit/19a6f3168a5e9719962ab389266af27cb2192aca))
- 计算出每一个节点的所有样式规则 ([0082034](https://github.com/NervJS/parse-css-to-stylesheet/commit/008203446135c8c6dbc7795bce957a8826d0bb47))
- 将 ast 中的 jsx tree 转为 ego tree ([34a1be5](https://github.com/NervJS/parse-css-to-stylesheet/commit/34a1be58c9466c6f6b1a141efbd225268d10ce89))
- 解析 JSX 构建 ego tree ([949a1ec](https://github.com/NervJS/parse-css-to-stylesheet/commit/949a1ecf1fc9f660e998db8e68be374786e1717a))
- 实现根据选择器查找 rust 文本节点，同时梳理代码 ([e850496](https://github.com/NervJS/parse-css-to-stylesheet/commit/e8504965842280b86a9f23d3b0c6040bf6123343))
- 实现样式继承 ([d508d15](https://github.com/NervJS/parse-css-to-stylesheet/commit/d508d1569878ec41abb1ca2d1567c39a8108258b))
- 增加 ElementRef ([710d95c](https://github.com/NervJS/parse-css-to-stylesheet/commit/710d95c199e15641bf05e04bea0b6ca5965f4bca))
- 增加 style parser ([a4e3f11](https://github.com/NervJS/parse-css-to-stylesheet/commit/a4e3f11eb9bc3e909c616cee03865304e304c9f1))
- 增加对 JSX 子树的支持 ([c73bbe5](https://github.com/NervJS/parse-css-to-stylesheet/commit/c73bbe5e4f9234fed2b582de1f1883dc84cd0d3d))
- 增加记录每段样式的特异性 ([7c6b9c3](https://github.com/NervJS/parse-css-to-stylesheet/commit/7c6b9c3c49109fc1dbcf6a63ce54839bec748c73))
- 支持 JSX 节点没有 style 属性情况下，将节点对应的样式写入到 style 属性中 ([d9eb803](https://github.com/NervJS/parse-css-to-stylesheet/commit/d9eb803259e895c8332e379aae0209e184f37a19))
- 支持 React.Fragment 用法 ([ac404c8](https://github.com/NervJS/parse-css-to-stylesheet/commit/ac404c8d69f9a0ca252fb9c12b5b0d3ba6e410a9))
- 支持函数调用方式的子 JSX 拆分写法 ([73a5bcb](https://github.com/NervJS/parse-css-to-stylesheet/commit/73a5bcbf34bea9e3301ab0f3d88fa9f1de561ad2))
- 支持将样式节点记录写入 ast 中 ([c35cbdf](https://github.com/NervJS/parse-css-to-stylesheet/commit/c35cbdf15e02a773cc912b1eedeaf29922225ac4))
- 支持将样式文件中样式与 JSX 节点的 style 属性值合并 ([1beb45a](https://github.com/NervJS/parse-css-to-stylesheet/commit/1beb45a114c2a649bd46417afe0efb8d74b85f91))
- 支持类组件 ([416e595](https://github.com/NervJS/parse-css-to-stylesheet/commit/416e59575c1ad6c58f19e2cd6f34cec3b38db436))
- 支持为 taro 组件库引入组件增加样式 ([6eb025f](https://github.com/NervJS/parse-css-to-stylesheet/commit/6eb025f1024d09091e065baa69efe4dbe1c601e8))
- update ([ca39a94](https://github.com/NervJS/parse-css-to-stylesheet/commit/ca39a94e3da22b737b079cf34e9383591313519c))

## 0.0.3 (2023-10-11)

### Bug Fixes

- 修复 JSX 解析报错问题 ([7818a16](https://github.com/NervJS/parse-css-to-stylesheet/commit/7818a163712adada628ec3ece35c2a7c18a7b46d))

### Features

- 处理不可继承的样式的情况，同时支持样式值为 inherit 的情况 ([13ae716](https://github.com/NervJS/parse-css-to-stylesheet/commit/13ae7167d4cbeb6fc880d764301320909530f236))
- 代码解析支持装饰器 ([c5d8522](https://github.com/NervJS/parse-css-to-stylesheet/commit/c5d852215a8f98fa5137e691b9343232280a1587))
- 调整代码 ([9c5f5d7](https://github.com/NervJS/parse-css-to-stylesheet/commit/9c5f5d7b2f1269b39b9a90f446f6262c16c93d6d))
- 根据选择器特异性排序计算出每个节点的最终样式 ([070ce57](https://github.com/NervJS/parse-css-to-stylesheet/commit/070ce57bf24a197e614a2885913818e46ec4be50))
- 基于 napi-rs 改造 ([19a6f31](https://github.com/NervJS/parse-css-to-stylesheet/commit/19a6f3168a5e9719962ab389266af27cb2192aca))
- 计算出每一个节点的所有样式规则 ([0082034](https://github.com/NervJS/parse-css-to-stylesheet/commit/008203446135c8c6dbc7795bce957a8826d0bb47))
- 将 ast 中的 jsx tree 转为 ego tree ([34a1be5](https://github.com/NervJS/parse-css-to-stylesheet/commit/34a1be58c9466c6f6b1a141efbd225268d10ce89))
- 解析 JSX 构建 ego tree ([949a1ec](https://github.com/NervJS/parse-css-to-stylesheet/commit/949a1ecf1fc9f660e998db8e68be374786e1717a))
- 实现根据选择器查找 rust 文本节点，同时梳理代码 ([e850496](https://github.com/NervJS/parse-css-to-stylesheet/commit/e8504965842280b86a9f23d3b0c6040bf6123343))
- 实现样式继承 ([d508d15](https://github.com/NervJS/parse-css-to-stylesheet/commit/d508d1569878ec41abb1ca2d1567c39a8108258b))
- 增加 ElementRef ([710d95c](https://github.com/NervJS/parse-css-to-stylesheet/commit/710d95c199e15641bf05e04bea0b6ca5965f4bca))
- 增加 style parser ([a4e3f11](https://github.com/NervJS/parse-css-to-stylesheet/commit/a4e3f11eb9bc3e909c616cee03865304e304c9f1))
- 增加对 JSX 子树的支持 ([c73bbe5](https://github.com/NervJS/parse-css-to-stylesheet/commit/c73bbe5e4f9234fed2b582de1f1883dc84cd0d3d))
- 增加记录每段样式的特异性 ([7c6b9c3](https://github.com/NervJS/parse-css-to-stylesheet/commit/7c6b9c3c49109fc1dbcf6a63ce54839bec748c73))
- 支持 JSX 节点没有 style 属性情况下，将节点对应的样式写入到 style 属性中 ([d9eb803](https://github.com/NervJS/parse-css-to-stylesheet/commit/d9eb803259e895c8332e379aae0209e184f37a19))
- 支持 React.Fragment 用法 ([ac404c8](https://github.com/NervJS/parse-css-to-stylesheet/commit/ac404c8d69f9a0ca252fb9c12b5b0d3ba6e410a9))
- 支持函数调用方式的子 JSX 拆分写法 ([73a5bcb](https://github.com/NervJS/parse-css-to-stylesheet/commit/73a5bcbf34bea9e3301ab0f3d88fa9f1de561ad2))
- 支持将样式节点记录写入 ast 中 ([c35cbdf](https://github.com/NervJS/parse-css-to-stylesheet/commit/c35cbdf15e02a773cc912b1eedeaf29922225ac4))
- 支持将样式文件中样式与 JSX 节点的 style 属性值合并 ([1beb45a](https://github.com/NervJS/parse-css-to-stylesheet/commit/1beb45a114c2a649bd46417afe0efb8d74b85f91))
- 支持类组件 ([416e595](https://github.com/NervJS/parse-css-to-stylesheet/commit/416e59575c1ad6c58f19e2cd6f34cec3b38db436))
- update ([ca39a94](https://github.com/NervJS/parse-css-to-stylesheet/commit/ca39a94e3da22b737b079cf34e9383591313519c))

## 0.0.2 (2023-10-11)

### Bug Fixes

- 修复 JSX 解析报错问题 ([7818a16](https://github.com/NervJS/parse-css-to-stylesheet/commit/7818a163712adada628ec3ece35c2a7c18a7b46d))

### Features

- 处理不可继承的样式的情况，同时支持样式值为 inherit 的情况 ([13ae716](https://github.com/NervJS/parse-css-to-stylesheet/commit/13ae7167d4cbeb6fc880d764301320909530f236))
- 代码解析支持装饰器 ([c5d8522](https://github.com/NervJS/parse-css-to-stylesheet/commit/c5d852215a8f98fa5137e691b9343232280a1587))
- 调整代码 ([9c5f5d7](https://github.com/NervJS/parse-css-to-stylesheet/commit/9c5f5d7b2f1269b39b9a90f446f6262c16c93d6d))
- 根据选择器特异性排序计算出每个节点的最终样式 ([070ce57](https://github.com/NervJS/parse-css-to-stylesheet/commit/070ce57bf24a197e614a2885913818e46ec4be50))
- 基于 napi-rs 改造 ([19a6f31](https://github.com/NervJS/parse-css-to-stylesheet/commit/19a6f3168a5e9719962ab389266af27cb2192aca))
- 计算出每一个节点的所有样式规则 ([0082034](https://github.com/NervJS/parse-css-to-stylesheet/commit/008203446135c8c6dbc7795bce957a8826d0bb47))
- 将 ast 中的 jsx tree 转为 ego tree ([34a1be5](https://github.com/NervJS/parse-css-to-stylesheet/commit/34a1be58c9466c6f6b1a141efbd225268d10ce89))
- 解析 JSX 构建 ego tree ([949a1ec](https://github.com/NervJS/parse-css-to-stylesheet/commit/949a1ecf1fc9f660e998db8e68be374786e1717a))
- 实现根据选择器查找 rust 文本节点，同时梳理代码 ([e850496](https://github.com/NervJS/parse-css-to-stylesheet/commit/e8504965842280b86a9f23d3b0c6040bf6123343))
- 实现样式继承 ([d508d15](https://github.com/NervJS/parse-css-to-stylesheet/commit/d508d1569878ec41abb1ca2d1567c39a8108258b))
- 增加 ElementRef ([710d95c](https://github.com/NervJS/parse-css-to-stylesheet/commit/710d95c199e15641bf05e04bea0b6ca5965f4bca))
- 增加 style parser ([a4e3f11](https://github.com/NervJS/parse-css-to-stylesheet/commit/a4e3f11eb9bc3e909c616cee03865304e304c9f1))
- 增加对 JSX 子树的支持 ([c73bbe5](https://github.com/NervJS/parse-css-to-stylesheet/commit/c73bbe5e4f9234fed2b582de1f1883dc84cd0d3d))
- 增加记录每段样式的特异性 ([7c6b9c3](https://github.com/NervJS/parse-css-to-stylesheet/commit/7c6b9c3c49109fc1dbcf6a63ce54839bec748c73))
- 支持 JSX 节点没有 style 属性情况下，将节点对应的样式写入到 style 属性中 ([d9eb803](https://github.com/NervJS/parse-css-to-stylesheet/commit/d9eb803259e895c8332e379aae0209e184f37a19))
- 支持 React.Fragment 用法 ([ac404c8](https://github.com/NervJS/parse-css-to-stylesheet/commit/ac404c8d69f9a0ca252fb9c12b5b0d3ba6e410a9))
- 支持函数调用方式的子 JSX 拆分写法 ([73a5bcb](https://github.com/NervJS/parse-css-to-stylesheet/commit/73a5bcbf34bea9e3301ab0f3d88fa9f1de561ad2))
- 支持将样式节点记录写入 ast 中 ([c35cbdf](https://github.com/NervJS/parse-css-to-stylesheet/commit/c35cbdf15e02a773cc912b1eedeaf29922225ac4))
- 支持将样式文件中样式与 JSX 节点的 style 属性值合并 ([1beb45a](https://github.com/NervJS/parse-css-to-stylesheet/commit/1beb45a114c2a649bd46417afe0efb8d74b85f91))
- 支持类组件 ([416e595](https://github.com/NervJS/parse-css-to-stylesheet/commit/416e59575c1ad6c58f19e2cd6f34cec3b38db436))
- update ([ca39a94](https://github.com/NervJS/parse-css-to-stylesheet/commit/ca39a94e3da22b737b079cf34e9383591313519c))

## 0.0.2 (2023-10-10)

### Features

- 处理不可继承的样式的情况，同时支持样式值为 inherit 的情况 ([13ae716](https://github.com/NervJS/parse-css-to-stylesheet/commit/13ae7167d4cbeb6fc880d764301320909530f236))
- 代码解析支持装饰器 ([c5d8522](https://github.com/NervJS/parse-css-to-stylesheet/commit/c5d852215a8f98fa5137e691b9343232280a1587))
- 调整代码 ([9c5f5d7](https://github.com/NervJS/parse-css-to-stylesheet/commit/9c5f5d7b2f1269b39b9a90f446f6262c16c93d6d))
- 根据选择器特异性排序计算出每个节点的最终样式 ([070ce57](https://github.com/NervJS/parse-css-to-stylesheet/commit/070ce57bf24a197e614a2885913818e46ec4be50))
- 基于 napi-rs 改造 ([19a6f31](https://github.com/NervJS/parse-css-to-stylesheet/commit/19a6f3168a5e9719962ab389266af27cb2192aca))
- 计算出每一个节点的所有样式规则 ([0082034](https://github.com/NervJS/parse-css-to-stylesheet/commit/008203446135c8c6dbc7795bce957a8826d0bb47))
- 将 ast 中的 jsx tree 转为 ego tree ([34a1be5](https://github.com/NervJS/parse-css-to-stylesheet/commit/34a1be58c9466c6f6b1a141efbd225268d10ce89))
- 解析 JSX 构建 ego tree ([949a1ec](https://github.com/NervJS/parse-css-to-stylesheet/commit/949a1ecf1fc9f660e998db8e68be374786e1717a))
- 实现根据选择器查找 rust 文本节点，同时梳理代码 ([e850496](https://github.com/NervJS/parse-css-to-stylesheet/commit/e8504965842280b86a9f23d3b0c6040bf6123343))
- 实现样式继承 ([d508d15](https://github.com/NervJS/parse-css-to-stylesheet/commit/d508d1569878ec41abb1ca2d1567c39a8108258b))
- 增加 ElementRef ([710d95c](https://github.com/NervJS/parse-css-to-stylesheet/commit/710d95c199e15641bf05e04bea0b6ca5965f4bca))
- 增加 style parser ([a4e3f11](https://github.com/NervJS/parse-css-to-stylesheet/commit/a4e3f11eb9bc3e909c616cee03865304e304c9f1))
- 增加对 JSX 子树的支持 ([c73bbe5](https://github.com/NervJS/parse-css-to-stylesheet/commit/c73bbe5e4f9234fed2b582de1f1883dc84cd0d3d))
- 增加记录每段样式的特异性 ([7c6b9c3](https://github.com/NervJS/parse-css-to-stylesheet/commit/7c6b9c3c49109fc1dbcf6a63ce54839bec748c73))
- 支持 JSX 节点没有 style 属性情况下，将节点对应的样式写入到 style 属性中 ([d9eb803](https://github.com/NervJS/parse-css-to-stylesheet/commit/d9eb803259e895c8332e379aae0209e184f37a19))
- 支持 React.Fragment 用法 ([ac404c8](https://github.com/NervJS/parse-css-to-stylesheet/commit/ac404c8d69f9a0ca252fb9c12b5b0d3ba6e410a9))
- 支持函数调用方式的子 JSX 拆分写法 ([73a5bcb](https://github.com/NervJS/parse-css-to-stylesheet/commit/73a5bcbf34bea9e3301ab0f3d88fa9f1de561ad2))
- 支持将样式节点记录写入 ast 中 ([c35cbdf](https://github.com/NervJS/parse-css-to-stylesheet/commit/c35cbdf15e02a773cc912b1eedeaf29922225ac4))
- 支持将样式文件中样式与 JSX 节点的 style 属性值合并 ([1beb45a](https://github.com/NervJS/parse-css-to-stylesheet/commit/1beb45a114c2a649bd46417afe0efb8d74b85f91))
- 支持类组件 ([416e595](https://github.com/NervJS/parse-css-to-stylesheet/commit/416e59575c1ad6c58f19e2cd6f34cec3b38db436))
- update ([ca39a94](https://github.com/NervJS/parse-css-to-stylesheet/commit/ca39a94e3da22b737b079cf34e9383591313519c))

## 0.0.1 (2023-10-10)

### Features

- 处理不可继承的样式的情况，同时支持样式值为 inherit 的情况 ([13ae716](https://github.com/NervJS/parse-css-to-stylesheet/commit/13ae7167d4cbeb6fc880d764301320909530f236))
- 根据选择器特异性排序计算出每个节点的最终样式 ([070ce57](https://github.com/NervJS/parse-css-to-stylesheet/commit/070ce57bf24a197e614a2885913818e46ec4be50))
- 基于 napi-rs 改造 ([19a6f31](https://github.com/NervJS/parse-css-to-stylesheet/commit/19a6f3168a5e9719962ab389266af27cb2192aca))
- 计算出每一个节点的所有样式规则 ([0082034](https://github.com/NervJS/parse-css-to-stylesheet/commit/008203446135c8c6dbc7795bce957a8826d0bb47))
- 将 ast 中的 jsx tree 转为 ego tree ([34a1be5](https://github.com/NervJS/parse-css-to-stylesheet/commit/34a1be58c9466c6f6b1a141efbd225268d10ce89))
- 解析 JSX 构建 ego tree ([949a1ec](https://github.com/NervJS/parse-css-to-stylesheet/commit/949a1ecf1fc9f660e998db8e68be374786e1717a))
- 实现根据选择器查找 rust 文本节点，同时梳理代码 ([e850496](https://github.com/NervJS/parse-css-to-stylesheet/commit/e8504965842280b86a9f23d3b0c6040bf6123343))
- 实现样式继承 ([d508d15](https://github.com/NervJS/parse-css-to-stylesheet/commit/d508d1569878ec41abb1ca2d1567c39a8108258b))
- 增加 ElementRef ([710d95c](https://github.com/NervJS/parse-css-to-stylesheet/commit/710d95c199e15641bf05e04bea0b6ca5965f4bca))
- 增加 style parser ([a4e3f11](https://github.com/NervJS/parse-css-to-stylesheet/commit/a4e3f11eb9bc3e909c616cee03865304e304c9f1))
- 增加对 JSX 子树的支持 ([c73bbe5](https://github.com/NervJS/parse-css-to-stylesheet/commit/c73bbe5e4f9234fed2b582de1f1883dc84cd0d3d))
- 增加记录每段样式的特异性 ([7c6b9c3](https://github.com/NervJS/parse-css-to-stylesheet/commit/7c6b9c3c49109fc1dbcf6a63ce54839bec748c73))
- 支持 JSX 节点没有 style 属性情况下，将节点对应的样式写入到 style 属性中 ([d9eb803](https://github.com/NervJS/parse-css-to-stylesheet/commit/d9eb803259e895c8332e379aae0209e184f37a19))
- 支持 React.Fragment 用法 ([ac404c8](https://github.com/NervJS/parse-css-to-stylesheet/commit/ac404c8d69f9a0ca252fb9c12b5b0d3ba6e410a9))
- 支持函数调用方式的子 JSX 拆分写法 ([73a5bcb](https://github.com/NervJS/parse-css-to-stylesheet/commit/73a5bcbf34bea9e3301ab0f3d88fa9f1de561ad2))
- 支持将样式节点记录写入 ast 中 ([c35cbdf](https://github.com/NervJS/parse-css-to-stylesheet/commit/c35cbdf15e02a773cc912b1eedeaf29922225ac4))
- 支持将样式文件中样式与 JSX 节点的 style 属性值合并 ([1beb45a](https://github.com/NervJS/parse-css-to-stylesheet/commit/1beb45a114c2a649bd46417afe0efb8d74b85f91))
- 支持类组件 ([416e595](https://github.com/NervJS/parse-css-to-stylesheet/commit/416e59575c1ad6c58f19e2cd6f34cec3b38db436))
- update ([ca39a94](https://github.com/NervJS/parse-css-to-stylesheet/commit/ca39a94e3da22b737b079cf34e9383591313519c))
