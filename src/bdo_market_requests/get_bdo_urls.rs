pub fn get_market_url(region: &str) -> &str {
    match region {
        "na" => "https://na-trade.naeu.playblackdesert.com",
        "eu" => "https://eu-trade.naeu.playblackdesert.com",
        "sea" => "https://trade.sea.playblackdesert.com",
        "mena" => "https://trade.tr.playblackdesert.com",
        "kr" => "https://trade.kr.playblackdesert.com",
        "ru" => "https://trade.ru.playblackdesert.com",
        "jp" => "https://trade.jp.playblackdesert.com",
        "th" => "https://trade.th.playblackdesert.com",
        "tw" => "https://trade.tw.playblackdesert.com",
        "sa" => "https://trade.sa.playblackdesert.com",
        "console_eu" => "https://eu-trade.console.playblackdesert.com",
        "console_na" => "https://na-trade.console.playblackdesert.com",
        "console_asia" => "https://asia-trade.console.playblackdesert.com",
        _ => panic!("Invalid region entered!"),
    }
}

pub fn create_post_url(region_url: &str, post_req_subdirectory: &str) -> String {
    let mut url = region_url.to_owned();
    url.push_str(post_req_subdirectory);
    url
}
