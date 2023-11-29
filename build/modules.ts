import geoJson from "./geo-china.json";
const geoChina = geoJson;
export { geoChina };

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
