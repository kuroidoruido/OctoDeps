use crate::models::AssetInfos;

pub fn get_asset_info(url: String) -> Result<Vec<AssetInfos>,reqwest::Error> {
    return reqwest::blocking::get(url.as_str())?.json::<Vec<AssetInfos>>();
}