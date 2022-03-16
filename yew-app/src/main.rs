use yew::prelude::*;

enum Msg{
    AddOne,
    Substract,
}


struct Model{
    value: i64,
}


impl Component for Model{
    type Message=Msg;
    type Properties = ();
    
    fn create(_ctx: &Context<Self>)->Self{
        Self{
            value:0
        }
    }


    fn update(&mut self,_ctx: &Context<Self>, msg: Self::Message)->bool{
        match msg{
            Msg::AddOne=>{
                self.value +=1;
                true
            },
            Msg::Substract=>{
                self.value-=1;
                true
            }
        }
    }

    fn view(&self,ctx: &Context<Self>)->Html{
        let link = ctx.link();
        html!{
            <div class={"top-20 bg-white w-69 h-56 rounded-lg"}>
            <button class={"bg-slate-600 p-4 m-6 rounded-lg text-white hover:bg-slate-800 duration-500"} onclick={link.callback(|_|Msg::AddOne)} >{"Add One"}</button>
            <button class={"bg-slate-600 p-4 m-6 rounded-lg text-white hover:bg-slate-800 duration-500"} onclick={link.callback(|_|Msg::Substract)} >{"Substrack one"}</button>
            <p class={"flex justify-center mt-10 font-semibold text-3xl bg-slat-700"}>{self.value}</p>
            </div>
        }
    }
}


fn main() {
   yew::start_app::<Model>();
}
