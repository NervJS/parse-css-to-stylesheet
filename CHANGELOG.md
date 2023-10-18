## [0.0.12](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.11...v0.0.12) (2023-10-18)


### Bug Fixes

* 遗漏样式属性添加情况 ([b698459](https://github.com/NervJS/parse-css-to-stylesheet/commit/b698459608d91030b2c4b8ba28c46e15bbe08189))


### Features

* 开始尝试在在编译时按需处理样式 ([82aa6f5](https://github.com/NervJS/parse-css-to-stylesheet/commit/82aa6f5a3a715d7e3ac50ac784b3a19cd4fe06e6))
* 优化样式处理逻辑 ([d8954ae](https://github.com/NervJS/parse-css-to-stylesheet/commit/d8954aed25438151a7c48e477b462dd0235507d5))
* 支持优化处理 text-decoration 属性 ([9799f94](https://github.com/NervJS/parse-css-to-stylesheet/commit/9799f94e9136346e52e2029099b991bfb077f643))



## [0.0.11](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.10...v0.0.11) (2023-10-17)



## [0.0.10](https://github.com/NervJS/parse-css-to-stylesheet/compare/v0.0.8...v0.0.10) (2023-10-17)


### Bug Fixes

* __inner_style__ 要符合 JSON 定义 ([94a533e](https://github.com/NervJS/parse-css-to-stylesheet/commit/94a533eff0531453f8932a74e3672a3d16fe934c))
* 避免将 rgab 色值转为 hexalpha ([ccbe771](https://github.com/NervJS/parse-css-to-stylesheet/commit/ccbe771da730911712df300cddf4479437e5eb80))
* 将样式名转为 camelCase ([cc39775](https://github.com/NervJS/parse-css-to-stylesheet/commit/cc3977548b3a0fb898cc499b6a2d493d19d1a491))



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



## [0.0.5](https://github.com/NervJS/parse-css-to-stylesheet/compare/0.0.4...0.0.5) (2023-10-11)


### Bug Fixes

* 修复 jsx 遍历不完整问题 ([cec4e20](https://github.com/NervJS/parse-css-to-stylesheet/commit/cec4e20710ec6f621cdff93961e5df3897999dd1))


### Features

* 去掉 typescript 声明 ([3c68159](https://github.com/NervJS/parse-css-to-stylesheet/commit/3c68159872f082f55081a978d4da5bc8805abf69))



## 0.0.4 (2023-10-11)


### Bug Fixes

* 修复 JSX 解析报错问题 ([7818a16](https://github.com/NervJS/parse-css-to-stylesheet/commit/7818a163712adada628ec3ece35c2a7c18a7b46d))


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
* 支持为 taro 组件库引入组件增加样式 ([6eb025f](https://github.com/NervJS/parse-css-to-stylesheet/commit/6eb025f1024d09091e065baa69efe4dbe1c601e8))
* update ([ca39a94](https://github.com/NervJS/parse-css-to-stylesheet/commit/ca39a94e3da22b737b079cf34e9383591313519c))



## 0.0.3 (2023-10-11)


### Bug Fixes

* 修复 JSX 解析报错问题 ([7818a16](https://github.com/NervJS/parse-css-to-stylesheet/commit/7818a163712adada628ec3ece35c2a7c18a7b46d))


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



## 0.0.2 (2023-10-11)


### Bug Fixes

* 修复 JSX 解析报错问题 ([7818a16](https://github.com/NervJS/parse-css-to-stylesheet/commit/7818a163712adada628ec3ece35c2a7c18a7b46d))


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



## 0.0.2 (2023-10-10)


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



## 0.0.1 (2023-10-10)


### Features

* 处理不可继承的样式的情况，同时支持样式值为 inherit 的情况 ([13ae716](https://github.com/NervJS/parse-css-to-stylesheet/commit/13ae7167d4cbeb6fc880d764301320909530f236))
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



