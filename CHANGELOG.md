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



