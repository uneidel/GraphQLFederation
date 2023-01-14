#[allow(non_snake_case)]
#[allow(unused_variables)]
pub mod PCNModel {
    
    use async_graphql::*;

    #[derive(SimpleObject)]
    struct Subgraph1Data<'a> {
        Hotel: &'a str,
        Room: &'a str,
        Date: &'a str,        
    }

    pub struct Query;
    #[Object]
    impl Query {
        async fn Hotel<'ctx>(
            &self,
            ctx: &async_graphql::Context<'ctx>,
            #[graphql(desc = "PCNId")] pcnid: String,
            #[graphql(desc = "From Date e.g. Dec-12-2018")] from_date: MaybeUndefined<String>,
            #[graphql(desc = "To Date e.g. Dec-12-2018")] to_date: MaybeUndefined<String>,
            #[graphql(desc = "PageSize")] page_size: MaybeUndefined<i32>,
            #[graphql(desc = "Offset")] offset: MaybeUndefined<i32>,
        ) -> Vec<Subgraph1Data<'ctx>> {
            let mut v: Vec<Subgraph1Data> = Vec::new();
            //Implement Filter 
            let l = Subgraph1Data {
                Hotel: "Hotel1",
                Room: "101",
                Date: "Entersomedate",
            };
            let x = Subgraph1Data {
                Hotel: "Hotel2",
                Room: "102",
                Date: "EnterSomeDate",
            };
            v.push(l);
            v.push(x);

            v
        }
    }
}
