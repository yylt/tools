
use may::{go};
use std::time::Duration;
use eframe::egui::{self, Context, Pos2};
use crossbeam::channel::{after, select, Sender,unbounded};
use enigo::{Enigo, Keyboard,Settings, Mouse, Direction::*,Button};
use enigo::Coordinate::*;
use log::{debug, error, info};
use std::sync::{Arc, RwLock};

#[derive( Debug)]
pub struct Nosleep {
    label: String,

    sender: Sender<Event>,

    // emulate
    em: enigo::Enigo,
    // interval
    value:  String,
    x: String,
    y: String,
}

#[derive(Debug)]
struct Event {
    du: Duration,
    x: String,
    y: String,
}

impl Default for Nosleep {
    fn default() -> Self {
        let (s1,r1) = unbounded::<Event>();
        // let point =  Arc::new(RwLock::new("0, 0".to_string()));
        // let spoint = Arc::clone(&point);
        go!(move || {
            let mut du = Duration::MAX;
            let mut enigo = Enigo::new(&Settings::default()).unwrap();
            let (mut x, mut y) = (0,0);
            loop {
                select!{
                    recv(after(du)) -> _ => {
                        enigo.move_mouse(x ,y, Abs).unwrap();  
                        info!("mouse left click: {:?}", enigo.button(Button::Left, Click));
                        info!("key click: {:?}",enigo.key(enigo::Key::Unicode('a'), Click));
                    },
                    recv(r1) -> tm => {
                        info!("received: {:?}",tm);
                        if let Ok(ev) = tm {
                            if ev.du.as_secs() == 0{
                                du = Duration::MAX;
                            }else{
                                du = ev.du;
                            }
                            x = ev.x.parse::<i32>().unwrap();
                            y = ev.y.parse::<i32>().unwrap();
                        }else{
                            error!("receive error, exit");
                            return
                        }
                    }
                }
            }
        });
       
        Self {
            em: Enigo::new(&Settings::default()).unwrap(),
            label: String::from("启动"),
            sender: s1,
            value: "10".to_owned(),
            x: String::from(""),
            y: String::from(""),
        }

    }
}

impl Nosleep {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for Nosleep {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        egui::TopBottomPanel::top("no sleep").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                egui::widgets::global_dark_light_mode_buttons(ui);
            });

            ui.horizontal(|ui| {
                ui.label(String::from("周期(s): "));
                ui.add(egui::TextEdit::singleline(&mut self.value).desired_width(40.0));
                ui.label(String::from("x坐标: "));
                ui.add(egui::TextEdit::singleline(&mut self.x).desired_width(40.0));
                ui.label(String::from("y坐标: "));
                ui.add(egui::TextEdit::singleline(&mut self.y).desired_width(40.0));

                if ui.button(&self.label).clicked() {
                    if self.label.contains("启") {
                        self.label = "停止".to_owned();
                    }else{
                        self.label = String::from("启动");
                        let _ = self.sender.send(Event {du: Duration::new(0,0),x:self.x.clone(),y:self.y.clone()});
                        return
                    }
                    match self.value.parse::<u64>() {
                        Ok(v) => {
                            let _ = self.sender.send( Event {du: Duration::new(v,0),x:self.x.clone(),y:self.y.clone() } );
                            debug!("发送 value: {:?}", v);
                        },
                        Err(e) => {
                            error!("wrong interval value: {:?}", e);
                        }
                    }
                }
            });
            ui.horizontal(|ui| {
                match ctx.input(|i| i.pointer.hover_pos()) {
                    _ => {
                        let (x, y)= self.em.location().unwrap();
                        ui.label(format!("当前位置: ({:.2}, {:.2})", x,y));
                    },
                }
            });
            
        });
    }
}
