use actix_web::{post, HttpResponse, Responder, web};
use crate::algo_hub::{algo_configurator::HammerPatternAlgoConfiguration, algo_backtester::AlgoBacktester};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct HammerAlgoParams{
    algo_configuration: HammerPatternAlgoConfiguration
}

//TODO: might be useful when we do live trading
// #[post("/algo/run_hammer_pattern")]
// async fn run_hammer_pattern(hammer_algo_params: web::Json<HammerAlgoParams>) -> impl Responder {

//     let result: Option<String> = None;
//     println!("hammer_algo_params body: {:?}", hammer_algo_params);
//     let algo_configurator = AlgoConfigurator{
//         algo_type: AlgoTypes::HammerPatternAlgo,
//         algo_metadata: AlgoConfigurationMetadata::HammerPatternMetadata(hammer_algo_params.algo_configuration.clone())
//     };
//     algo_configurator.initiate_the_backtest();
//     if result.is_some(){
         
//         HttpResponse::Ok().json(result)
//     }else{
//         HttpResponse::Ok().body("No hammer candles found")
//     }
// }



#[post("/algo/backtest_strategy/{pnl_configuration_id}")]
async fn backtest_strategy(path: web::Path<String>) -> impl Responder {
    // let mut result: Option<String> = None;
    let pnl_configuration_id = path.into_inner();
    println!("API pnl_configuration_id received: {:?}", pnl_configuration_id);
    let algo_backtester = AlgoBacktester::new(pnl_configuration_id);
    let result = algo_backtester.initiate_the_backtest().await;
    if result.is_some(){
        HttpResponse::Ok().json(result.unwrap())
    }else{
        HttpResponse::Ok().body("No hammer candles found")
    }
}