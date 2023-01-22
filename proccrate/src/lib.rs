/* 
Use this ProcMacro to load Configuration 
enable proc-macro=true in config.toml 

#[uneidelmacro::uneidelconfig(Dev="./config/Development.toml", Prod="./Production.toml")]
pub struct Settings {
    security: Security,
    server: Server
} 

let settings = Settings::new()

-############################

*/
use proc_macro::TokenStream;
use quote::{quote};
use syn::{AttributeArgs, Item, NestedMeta, Lit,Meta};
use std::collections::HashMap;

#[proc_macro_attribute]
pub fn uneidelconfig(_args: TokenStream, _item: TokenStream) -> TokenStream {
    let args = syn::parse_macro_input!(_args as AttributeArgs);
    
    let h = getkeyval(args);
    let mut DevPath: String = String::new();
    let mut ProdPath: String = String::new();
    if h.contains_key("dev"){
        DevPath = h["dev"].to_string();
    }
    if h.contains_key("prod"){
        ProdPath = h["prod"].to_string();
    }

    let item = syn::parse_macro_input!(_item as Item);
    let _xxx = match item.to_owned() {
        Item::Struct(s) => {
            println!("Its a struct");
            s
        }
        _ => todo!()
    };
    

    quote! {
        use config::{Config,ConfigError};
        #[derive(Deserialize, Clone)]
        #item

        impl Settings {
            pub fn new() -> Result<Self, ConfigError> {
                let env = std::env::var("env").unwrap_or_else(|_| "Development".into());
                let mut configpath =#DevPath;
                if !cfg!(debug_assertions) ||  env == "Production" {
                    configpath = #ProdPath;
                }
                println!("ConfigPath: {:?}", configpath);
                let settings = Config::builder()
                .add_source(config::File::with_name(configpath))
                // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
                .add_source(config::Environment::with_prefix("env"))
                .set_override("env", "Development")?
                .build()?
                .try_deserialize::<Settings>();


                return settings;
            }
        }
    }
    .into()
}


fn getkeyval(args: Vec<NestedMeta>) -> HashMap<String,String>{
     let mut h: HashMap<String, String> = HashMap::new();
    
    for nested_meta in args {
        let meta = match nested_meta {
            NestedMeta::Meta(m) => m,
            _ => todo!("wrong meta type"),
        };
        let meta = match meta {
            Meta::NameValue(n) => n,
            _ => todo!()
        };
        let value = match &meta.lit {
            Lit::Str(s) => s.value(),
            _ => todo!("wrong meta type"),
        };
    
        let ident = match meta.path.get_ident() {
            None => todo!("missing ident"),
            Some(ident) => ident,
        };
        println!("Ident: {:?}, value:{:?}", ident.to_string(), value);
        h.insert(ident.to_string().to_lowercase(), value);

    }
    h
}



