pub mod about;
pub mod home;

use yew_router::prelude::*;

#[derive(Debug, Switch, Clone)]
pub enum AppRoute { 
    #[to = "/about"]
    About,
    #[to = "/"]
    Home,
   
}