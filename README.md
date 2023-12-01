<br>
<p align="center">
  <a href="" target="_blank">
    <img src="https://xiaoxian521.github.io/hyperlink/img/esmjs-geo.png" alt="@esmjs/geo  " width="220" />
  </a>
</p>

<p align="center">
@esmjs/geo  
<br />
中国地理信息（GeoJSON）
</p>

<p align="center">
<a href="https://www.npmjs.com/package/@esmjs/geo" target="__blank"><img src="https://img.shields.io/npm/v/@esmjs/geo?color=67C23A&label=" alt="NPM version"></a>
</p>

## 🚀 特性

- 🦀 **`rust`爬虫**: 使用`rust`现代系统编程语言编写爬虫来获取中国地理信息，安全性高、速度快并且并发性高
- 🐹 **极快打包**: 使用`esbuild`打包，本质使用`go`语言打包，拥有极快的打包速度
- ⚡️ **完全可摇树**: 自带`Tree-shaking`，只对引入的代码进行打包
- 💫 **零依赖**: 零`Dependencies`依赖，只会安装项目本身
- 🦾 **强类型**: 使用`TypeScript`编写，拥有强大的类型推导提示

## 📦 安装

```bash
# npm
npm install @esmjs/geo

# or yarn
yarn add @esmjs/geo

# or pnpm
pnpm add @esmjs/geo
```

## 📕 用法

```ts
// 如搭配 ECharts 地图组件，@esmjs/geo 还支持中国全省数据，请看下面的速查表
// import { china as geoChina } from "@esmjs/geo"; // 如有命名冲突使用 as 别名即可
import { china } from "@esmjs/geo";
import * as echarts from "echarts/core";

/**
 * 由于 echarts 没有导出 geoJSON 的 GeoJSONSourceInput 类型
 * china 只能导出标准的 GeoJSON 格式类型
 * 如果你的项目是 TypeScript 编写，下面代码加上 //@ts-expect-error 即可
 */
echarts.registerMap("china", { geoJSON: china });
```

## 🌍 使用场景

常用于 [ECharts 地图组件](https://echarts.apache.org/handbook/zh/basics/release-note/5-3-0/#registermap-%E5%92%8C-getmap-%E6%96%B9%E6%B3%95%E9%9C%80%E8%A6%81%E5%9C%A8%E5%BC%95%E5%85%A5%E5%9C%B0%E5%9B%BE%E7%BB%84%E4%BB%B6%E5%90%8E%E6%89%8D%E8%83%BD%E4%BD%BF%E7%94%A8)

<img src="https://xiaoxian521.github.io/hyperlink/img/echarts-geo.jpg" alt="echarts-geo" width="340" />

## 🇨🇳 速查表

一个中国，`34`个省级行政区，`23`个省、`5`个自治区、`4`个直辖市、`2`个特别行政区

|   中国   |  **陕西**  |   山西    |   辽宁    |   吉林   |    黑龙江    |
| :------: | :--------: | :-------: | :-------: | :------: | :----------: |
|  china   |  shaanxi   |  shanxi   | liaoning  |  jilin   | heilongjiang |
| **江苏** |  **浙江**  | **安徽**  | **福建**  | **江西** |   **山东**   |
| jiangsu  |  zhejiang  |   anhui   |  fujian   | jiangxi  |   shandong   |
| **河南** |  **湖北**  | **湖南**  | **广东**  | **海南** |   **四川**   |
|  henan   |   hubei    |   hunan   | guangdong |  hainan  |   sichuan    |
| **贵州** |  **云南**  | **河北**  | **甘肃**  | **青海** |   **台湾**   |
| guizhou  |   yunnan   |   hebei   |   gansu   | qinghai  |    taiwan    |
| **广西** | **内蒙古** | **西藏**  | **宁夏**  | **新疆** |   **北京**   |
| guangxi  | neimenggu  |  xizang   |  ningxia  | xinjiang |   beijing    |
| **天津** |  **上海**  | **重庆**  | **香港**  | **澳门** |              |
| tianjin  |  shanghai  | chongqing | hongkong  |  aomen   |              |

## 🤔 FAQ

- [为什么使用`rust`编写爬虫程序](https://github.com/esmjs/geo/issues/1#issue-2015833595)
- [为什么使用`esbuild`打包而不是`rollup`](https://github.com/esmjs/geo/issues/1#issuecomment-1831371327)
- [既然是纯`esm`包，为什么`package.json`文件还要加`main`、`module`、`types`配置](https://github.com/esmjs/geo/issues/1#issuecomment-1831373645)
- [如何运行`rust`爬虫程序](https://github.com/esmjs/geo/issues/1#issuecomment-1831374543)
- [`rust`相关学习资料](https://github.com/esmjs/geo/issues/1#issuecomment-1831374680)
- [有么有类似`npmjs`的`rust`包管理器](https://github.com/esmjs/geo/issues/1#issuecomment-1831382508)
- [`rust`安装环境麻烦，如何通过`docker`运行`rust`爬虫程序](https://github.com/esmjs/geo/issues/1#issuecomment-1831445627)
- [地图数据来源](https://github.com/esmjs/geo/issues/1#issuecomment-1831535156)

## 许可证

[MIT © 2023-present, esmjs](./LICENSE)
