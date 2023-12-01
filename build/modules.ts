import China from "../json/china.json";
import Hebei from "../json/hebei.json";
import Shanxi from "../json/shanxi.json";
import Liaoning from "../json/liaoning.json";
import Jilin from "../json/jilin.json";
import Heilongjiang from "../json/heilongjiang.json";
import Jiangsu from "../json/jiangsu.json";
import Zhejiang from "../json/zhejiang.json";
import Anhui from "../json/anhui.json";
import Fujian from "../json/fujian.json";
import Jiangxi from "../json/jiangxi.json";
import Shandong from "../json/shandong.json";
import Henan from "../json/henan.json";
import Hubei from "../json/hubei.json";
import Hunan from "../json/hunan.json";
import Guangdong from "../json/guangdong.json";
import Hainan from "../json/hainan.json";
import Sichuan from "../json/sichuan.json";
import Guizhou from "../json/guizhou.json";
import Yunnan from "../json/yunnan.json";
import Shaanxi from "../json/shaanxi.json";
import Gansu from "../json/gansu.json";
import Qinghai from "../json/qinghai.json";
import Taiwan from "../json/taiwan.json";
import Guangxi from "../json/guangxi.json";
import Neimenggu from "../json/neimenggu.json";
import Xizang from "../json/xizang.json";
import Ningxia from "../json/ningxia.json";
import Xinjiang from "../json/xinjiang.json";
import Beijing from "../json/beijing.json";
import Tianjin from "../json/tianjin.json";
import Shanghai from "../json/shanghai.json";
import Chongqing from "../json/chongqing.json";
import Hongkong from "../json/hongkong.json";
import Aomen from "../json/aomen.json";

const china = China;
const hebei = Hebei;
const shanxi = Shanxi;
const liaoning = Liaoning;
const jilin = Jilin;
const heilongjiang = Heilongjiang;
const jiangsu = Jiangsu;
const zhejiang = Zhejiang;
const anhui = Anhui;
const fujian = Fujian;
const jiangxi = Jiangxi;
const shandong = Shandong;
const henan = Henan;
const hubei = Hubei;
const hunan = Hunan;
const guangdong = Guangdong;
const hainan = Hainan;
const sichuan = Sichuan;
const guizhou = Guizhou;
const yunnan = Yunnan;
const shaanxi = Shaanxi;
const gansu = Gansu;
const qinghai = Qinghai;
const taiwan = Taiwan;
const guangxi = Guangxi;
const neimenggu = Neimenggu;
const xizang = Xizang;
const ningxia = Ningxia;
const xinjiang = Xinjiang;
const beijing = Beijing;
const tianjin = Tianjin;
const shanghai = Shanghai;
const chongqing = Chongqing;
const hongkong = Hongkong;
const aomen = Aomen;

export {
  /** 中国（共34个省级行政区，23个省、5个自治区、4个直辖市以及2个特别行政区） */
  china,
  /** 河北省 */
  hebei,
  /** 山西省 */
  shanxi,
  /** 辽宁省 */
  liaoning,
  /** 吉林省 */
  jilin,
  /** 黑龙江省 */
  heilongjiang,
  /** 江苏省 */
  jiangsu,
  /** 浙江省 */
  zhejiang,
  /** 安徽省 */
  anhui,
  /** 福建省 */
  fujian,
  /** 江西省 */
  jiangxi,
  /** 山东省 */
  shandong,
  /** 河南省 */
  henan,
  /** 湖北省 */
  hubei,
  /** 湖南省 */
  hunan,
  /** 广东省 */
  guangdong,
  /** 海南省 */
  hainan,
  /** 四川省 */
  sichuan,
  /** 贵州省 */
  guizhou,
  /** 云南省 */
  yunnan,
  /** 陕西省 */
  shaanxi,
  /** 甘肃省 */
  gansu,
  /** 青海省 */
  qinghai,
  /** 台湾省 */
  taiwan,
  /** 广西壮族自治区 */
  guangxi,
  /** 内蒙古自治区 */
  neimenggu,
  /** 西藏自治区 */
  xizang,
  /** 宁夏回族自治区 */
  ningxia,
  /** 新疆维吾尔自治区 */
  xinjiang,
  /** 北京市 */
  beijing,
  /** 天津市 */
  tianjin,
  /** 上海市 */
  shanghai,
  /** 重庆市 */
  chongqing,
  /** 香港特别行政区 */
  hongkong,
  /** 澳门特别行政区 */
  aomen,
};

/** https://geojson.org/ */
export interface GEOJSON {
  type: string;
  geometry: {
    type: string;
    coordinates: Array<number>[];
  };
  properties: {
    name: string;
  };
}
