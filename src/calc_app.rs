/// We derive Deserialize/Serialize so we can persist app state on shutdown.
//type Error = Box<dyn std::error::Error>;


#[derive(serde::Deserialize, serde::Serialize)]
pub struct ExampleApp {
    equation: String,
   // show: bool,
}


impl Default for ExampleApp {
    fn default() -> Self {
        Self {
            equation: "".to_owned(),
            //show: false,
        }
    }
}

impl egui::app::App for ExampleApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.

    fn ui(
        &mut self,
        ctx: &std::sync::Arc<egui::Context>,
        integration_context: &mut egui::app::IntegrationContext,
    ) {
        let ExampleApp {
            equation,
            //show,
        } = self;

        // Example used in `README.md`.
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My Calculator");

            // ui.horizontal(|ui| {
            //     ui.label("Your name: ");
            //     ui.text_edit_singleline(name);
            //     ui.checkbox(show_label, "show label");

            //     *show_label ^= ui.button("flip").clicked
            // });
            

            // if name != "conor" && name != "anton" {
            //     ui.horizontal(|ui| {
            //         ui.label("Your name is not: conor");
            //     });
            // }
            // if *show_label {
            //     ui.label("hi");
            // }

            // ui.add(egui::Slider::u32(age, 0..=120).text("age"));
            // if ui.button("Click each year").clicked {
            //     *age += 1;
            // }

            // ui.label(format!("Hello '{}', age {}", name, *age - 1));

            // ui.add(egui::Slider::u32(length, 0..=200).text("längd cm"));
            ui.label("calculator");
            ui.text_edit_singleline(equation);
            ui.label(" ");

           // *show ^= ui.button("enter").clicked;
            ui.label(calculate(&equation).to_string());
            // if *show {
            //     ui.label(calculate(&equation).to_string());
            // }
            
            ui.advance_cursor(16.0);
            if ui.button("Quit").clicked {
                integration_context.output.quit = true;
            }
        });

        integration_context.output.window_size = Some(ctx.used_size()); // resize the window to be just the size we need it to be
    }

    fn on_exit(&mut self, storage: &mut dyn egui::app::Storage) {
        egui::app::set_value(storage, egui::app::APP_KEY, self);
    }
}

fn calculate(equation: &str)->f64{//things to add: support for pi, sqrt, sin, cos, tan, a result history, 
                                  //log, ln, e, maybe even i, error handling, graphs, sliding constants
    let text = &equation;
    let mut bracket_level;
    let start=0;
    let end=text.len();

    if text.len()!=0 && start <= end{
        bracket_level =0;
        for a in (start..end).rev(){
            let i: usize = a;
            if &text[i..i+1]==")"{
                bracket_level+=1;
            }
            if &text[i..i+1]=="("{
                bracket_level-=1;
            }
            //dbg!(&text[i..i+1], bracket_level);
            if (&text[i..i+1]== "-"|| &text[i..i+1]== "+") && bracket_level == 0{
                if &text[i..i+1]== "-"{
                    //dbg!("test", &*text, bracket_level);
                    return calculate(&text[start..a])-calculate(&text[a+1..end]);
                }else{
                    return calculate(&text[start..a])+calculate(&text[a+1..end]);
                }
            }
        }
        bracket_level =0;
        for a in (start..end).rev(){
            let i = a as usize;//what if i equals end???????? BUg
            if &text[i..i+1]==")"{
                bracket_level+=1;
            }
            if &text[i..i+1]=="("{
                bracket_level-=1;
            }
            if (&text[i..i+1] == "*"|| &text[i..i+1] == "/") && bracket_level == 0{
                if &text[i..i+1] == "*"{
                    return calculate(&text[start..a])*calculate(&text[a+1..end]);
                }else{
                    return calculate(&text[start..a])/calculate(&text[a+1..end]);
                }
            }
        }
        bracket_level =0;
        for a in (start..end).rev(){
            
            let i:usize = a;
            
                if &text[i..i+1]==")"{
                    bracket_level+=1;
                }
                if &text[i..i+1]=="("{
                    bracket_level-=1;
                }
                if &text[i..i+1] == "^" && bracket_level == 0{
                    return calculate(&text[start..a]).powf(calculate(&text[a+1..end]));
                }

        }
        dbg!(&text[start..start+1], &text[end-1..]);
        if &text[start..start+1]== "(" && &text[end-1..]==")"{
            return calculate(&text[start+1..end-1])
        }      
        bracket_level=0;
        for a in start..end{
            if &text[a..a+1]==")"{
                bracket_level+=1;
            }
            if &text[a..a+1]=="("{
                bracket_level-=1;
            }
        }
        if bracket_level== 0{
            let d: f64 =equation.parse().unwrap();
            return d as f64;
        }else {
            return 0.0
        }
    }
    return 0.0
}


// fn trim_calc(){

// }