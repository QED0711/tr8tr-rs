use crate::modules::data_transformer::{TransformerSet};
use crate::transformers;

#[allow(non_snake_case)]
pub fn SET_001() -> TransformerSet {

    let sma_args = transformers::moving_averages::SmaArgs{
        in_col: None, 
        out_col: Some("sma_50".to_string()), 
        period: Some(50)
    };
    
    let ema_args = transformers::moving_averages::EmaArgs{
        in_col: None, 
        out_col: Some("ema_50".to_string()),
        period: Some(50),
    };

    let ma_trend_args = transformers::moving_averages::TripleMaTrenendArgs{
        in_col: None,
        fast_period: Some(50),
        medium_period: Some(100),
        slow_period: Some(300),
        ma_type: Some("ema".to_string()),
    };

    let rsi_args = transformers::rsi::RsiArgs{
        in_col: None,
        out_col: Some("rsi".to_string()),
        period: Some(14),
    };

    let rsi_divergence_args = transformers::rsi::RsiDivergenceArgs{
        rsi_col: "rsi".to_string(), 
        out_col: Some("rsi_divergence".to_string()),
        lookback: Some(12),
        significance: Some(0.02),
    };

    let pivot_point_args = transformers::pivot_points::WeeklyPivotPointArgs{
        time_col: None,
        out_col_prefix: None,
    };    

    let atr_args = transformers::atr::AtrArgs{
        period: Some(14),
        out_col: Some("atr".to_string()),
    };

    let candle_atr_args = transformers::atr::CandleAtrArgs{
        atr_col: "atr".to_string(),
        low_level: Some("open".to_string()),
        high_level: Some("close".to_string()),
        out_col: Some("candle_atr".to_string()),
    };

    let candle_pattern_args = transformers::candle_patterns::CandlePatternArgs{
        color_out_col: Some("candle_color".to_string()),
        pattern_out_col: Some("candle_pattern".to_string()),
        score_out_col: Some("candle_pattern_score".to_string()),
    };

    let sma_50 = transformers::moving_averages::SMA(sma_args);
    let ema_50 = transformers::moving_averages::EMA(ema_args);
    let triple_ma_trend = transformers::moving_averages::TRIPLE_MA_TREND(ma_trend_args);
    let rsi_14 = transformers::rsi::RSI(rsi_args);
    let rsi_divergence = transformers::rsi::RSI_DIVERGENCE(rsi_divergence_args);
    let pivot_points = transformers::pivot_points::WEEKLY_PIVOT_POINTS(pivot_point_args);
    let atr = transformers::atr::ATR(atr_args);
    let candle_atr = transformers::atr::CANDLE_ATR(candle_atr_args);
    let candle_pattern = transformers::candle_patterns::CANDLE_PATTERN(candle_pattern_args);

    vec![
        Box::new(sma_50),
        Box::new(ema_50),
        Box::new(triple_ma_trend),
        Box::new(rsi_14),
        Box::new(rsi_divergence),
        Box::new(pivot_points),
        Box::new(atr),
        Box::new(candle_atr),
        Box::new(candle_pattern),
    ]
}