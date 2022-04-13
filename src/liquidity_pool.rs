use std::collections::HashMap;
use crate::factory::factory;
struct liquidity_pool<'a> {
    factory:factory<'a>,
    token0_address:&'a str,
    token1_address:&'a str,
    reserve0: f64,
    reserve1: f64,
    klast : f64,
    liquidity_providers :HashMap<&'a str,f64>,
    total_pool_tokens : f64

}

impl<'a> liquidity_pool<'a> {
    fn get_reserves(&self)-> (f64,f64){
        let reserve_0=self.reserve0;
        let reserve_1=self.reserve1;
        (reserve_0,reserve_1)
    }

    fn mint(&mut self, address_to:&'a str, token0_num:f64, token1_num:f64) -> f64{
        let result =(token0_num*token1_num).sqrt();
        if self.liquidity_providers.contains_key(address_to){
            let value =self.liquidity_providers.get(&address_to);
            let new_value= value.unwrap()+result;
            self.liquidity_providers.insert(address_to,new_value);
        }else{
            self.liquidity_providers.insert(address_to,result);
        }
        self.total_pool_tokens=self.total_pool_tokens+result;
        result

    }
    // tuple containing portions from token 0 and token 1
    fn burn (&mut self, address_from:&'a str, pool_tokens:f64) -> (f64, f64){
        let owned_pool_tokens = self.liquidity_providers.get(address_from).unwrap();
        assert!(pool_tokens<*owned_pool_tokens,"owned pool tokens are not enough");
        let token_ratio=pool_tokens/self.total_pool_tokens;
        let new_owned_tokens = owned_pool_tokens- pool_tokens;
        self.liquidity_providers.insert(address_from,new_owned_tokens);
        let  (token_0,token_1)=self.get_reserves();
        (token_ratio*token_0,token_ratio*token_1)
    }

    fn swap (&mut self ,token0_in:f64,token1_in:f64,address_to:&'a str)->f64{
        assert!(token0_in> 0.0 || token1_in>0.0, "invalid in-amount ");
        assert!(token0_in== 0.0 || token1_in==0.0, "only one token should be as input");
        let (reserve_0,reserve_1) = self.get_reserves();
        let mut token_out:f64=0.0;
        if(token0_in>0.0){
            let token_after_fees=token0_in*99.7/100.0;
            let token_1_reserves = self.klast/(token_after_fees+reserve_0);
            token_out=reserve_1-token_1_reserves;
            self.reserve0=reserve_0+token_after_fees;
            self.reserve1=token_1_reserves;
        }else{
            let token_after_fees=token1_in*99.7/100.0;
            let token_0_reserves = self.klast/(token_after_fees+reserve_0);
            token_out=reserve_0-token_0_reserves;
            self.reserve1=reserve_1+token_after_fees;
            self.reserve0=token_0_reserves;

        }
        token_out
    }



}