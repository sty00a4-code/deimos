use luna_rs::{
    function,
    lang::value::{FunctionKind, Object, UserObject, UserObjectError, Value},
    luna_impl::interpreter::Interpreter,
    object, option, set_field, typed, ExpectedType, ExpectedTypes,
};
use sdl2::{
    event::{DisplayEvent, Event, WindowEvent}, mouse::MouseWheelDirection, rect::Rect, render::Canvas, video::{FullscreenType, Orientation, Window}, EventPump, Sdl
};
use std::{cell::RefCell, collections::HashMap, error::Error, rc::Rc};

pub fn insert_module(globals: &mut HashMap<String, Rc<RefCell<Value>>>) {
    set_field!(globals."sdl" = object! {
        "init" = function!(_sdl_init)
    });
}

pub fn _sdl_init(_: &mut Interpreter, _: Vec<Value>) -> Result<Value, Box<dyn Error>> {
    Ok(Value::UserObject(Rc::new(RefCell::new(Box::new(
        SdlObject(sdl2::init()?),
    )))))
}

#[derive(Clone)]
pub struct SdlObject(Sdl);
impl UserObject for SdlObject {
    fn typ(&self) -> &'static str {
        "sdl"
    }
    fn get(&self, key: &str) -> Option<Value> {
        match key {
            "canvas" => Some(Value::Function(FunctionKind::UserFunction(Rc::new(
                Self::_canvas,
            )))),
            "events" => Some(Value::Function(FunctionKind::UserFunction(Rc::new(
                Self::_events,
            )))),
            _ => None,
        }
    }
    fn call(&self, key: &str, args: Vec<Value>) -> Result<Value, Box<dyn Error>> {
        match key {
            "canvas" => self.call_canvas(args),
            "events" => self.call_events(),
            _ => Err(UserObjectError::CannotCallNull.into()),
        }
    }
}
impl SdlObject {
    pub fn _canvas(_: &mut Interpreter, mut args: Vec<Value>) -> Result<Value, Box<dyn Error>> {
        let Some(_self) = args.first().cloned() else {
            return Err(Box::new(UserObjectError::ExpectedSelf("null")));
        };
        args.remove(0);
        if let Value::UserObject(_self) = _self {
            let mut _self = _self.borrow_mut();
            _self.call("canvas", args)
        } else {
            Err(Box::new(UserObjectError::ExpectedSelf(_self.typ())))
        }
    }
    pub fn call_canvas(&self, args: Vec<Value>) -> Result<Value, Box<dyn Error>> {
        let mut args = args.into_iter().enumerate();
        let title = typed!(args: String);
        let width = typed!(args: Int).try_into()?;
        let height = typed!(args: Int).try_into()?;
        let options = typed!(args: Object?);

        let mut window = self.0.video()?.window(&title, width, height).build()?;

        if let Some(options) = options {
            let options = options.borrow();
            for (option, value) in options.fields.iter() {
                match option.as_str() {
                    "always_on_top" => window.set_always_on_top(value.clone().into()),
                    "bordered" => window.set_bordered(value.clone().into()),
                    "brightness" => match value {
                        Value::Int(v) => window.set_brightness(*v as f64)?,
                        Value::Float(v) => window.set_brightness(*v)?,
                        _ => {}
                    },
                    "fullscreen" => window.set_fullscreen(if value.clone().into() {
                        FullscreenType::True
                    } else {
                        FullscreenType::Off
                    })?,
                    _ => {}
                };
            }
        }

        Ok(Value::UserObject(Rc::new(RefCell::new(Box::new(
            CanvasObject(window.into_canvas().build()?),
        )))))
    }
    pub fn _events(_: &mut Interpreter, mut args: Vec<Value>) -> Result<Value, Box<dyn Error>> {
        let Some(_self) = args.first().cloned() else {
            return Err(Box::new(UserObjectError::ExpectedSelf("null")));
        };
        args.remove(0);
        if let Value::UserObject(_self) = _self {
            let mut _self = _self.borrow_mut();
            _self.call("events", args)
        } else {
            Err(Box::new(UserObjectError::ExpectedSelf(_self.typ())))
        }
    }
    pub fn call_events(&self) -> Result<Value, Box<dyn Error>> {
        let event_pump = self.0.event_pump()?;
        Ok(Value::UserObject(Rc::new(RefCell::new(Box::new(
            EventPumpObject(event_pump),
        )))))
    }
}

pub struct CanvasObject(Canvas<Window>);
impl UserObject for CanvasObject {
    fn typ(&self) -> &'static str {
        "canvas"
    }
    fn get(&self, key: &str) -> Option<Value> {
        match key {
            "present" => Some(Value::Function(FunctionKind::UserFunction(Rc::new(
                Self::_present,
            )))),
            "clear" => Some(Value::Function(FunctionKind::UserFunction(Rc::new(
                Self::_clear,
            )))),
            "color" => Some(Value::Function(FunctionKind::UserFunction(Rc::new(
                Self::_color,
            )))),
            "scale" => Some(Value::Function(FunctionKind::UserFunction(Rc::new(
                Self::_scale,
            )))),
            "line" => Some(Value::Function(FunctionKind::UserFunction(Rc::new(
                Self::_line,
            )))),
            "point" => Some(Value::Function(FunctionKind::UserFunction(Rc::new(
                Self::_point,
            )))),
            "rect" => Some(Value::Function(FunctionKind::UserFunction(Rc::new(
                Self::_rect,
            )))),
            _ => None,
        }
    }
    fn call_mut(&mut self, key: &str, args: Vec<Value>) -> Result<Value, Box<dyn Error>> {
        match key {
            "present" => self.call_present(),
            "clear" => self.call_clear(),
            "color" => self.call_color(args),
            "scale" => self.call_scale(args),
            "line" => self.call_line(args),
            "point" => self.call_point(args),
            "rect" => self.call_rect(args),
            _ => Err(UserObjectError::CannotCallNull.into()),
        }
    }
}
impl CanvasObject {
    pub fn _present(_: &mut Interpreter, mut args: Vec<Value>) -> Result<Value, Box<dyn Error>> {
        let Some(_self) = args.first().cloned() else {
            return Err(Box::new(UserObjectError::ExpectedSelf("null")));
        };
        args.remove(0);
        if let Value::UserObject(_self) = _self {
            let mut _self = _self.borrow_mut();
            _self.call_mut("present", args)
        } else {
            Err(Box::new(UserObjectError::ExpectedSelf(_self.typ())))
        }
    }
    pub fn call_present(&mut self) -> Result<Value, Box<dyn Error>> {
        self.0.present();
        Ok(Value::default())
    }
    pub fn _clear(_: &mut Interpreter, mut args: Vec<Value>) -> Result<Value, Box<dyn Error>> {
        let Some(_self) = args.first().cloned() else {
            return Err(Box::new(UserObjectError::ExpectedSelf("null")));
        };
        args.remove(0);
        if let Value::UserObject(_self) = _self {
            let mut _self = _self.borrow_mut();
            _self.call_mut("clear", args)
        } else {
            Err(Box::new(UserObjectError::ExpectedSelf(_self.typ())))
        }
    }
    pub fn call_clear(&mut self) -> Result<Value, Box<dyn Error>> {
        self.0.clear();
        Ok(Value::default())
    }
    pub fn _color(_: &mut Interpreter, mut args: Vec<Value>) -> Result<Value, Box<dyn Error>> {
        let Some(_self) = args.first().cloned() else {
            return Err(Box::new(UserObjectError::ExpectedSelf("null")));
        };
        args.remove(0);
        if let Value::UserObject(_self) = _self {
            let mut _self = _self.borrow_mut();
            _self.call_mut("color", args)
        } else {
            Err(Box::new(UserObjectError::ExpectedSelf(_self.typ())))
        }
    }
    pub fn call_color(&mut self, args: Vec<Value>) -> Result<Value, Box<dyn Error>> {
        let mut args = args.into_iter().enumerate();
        let r = typed!(args: Int).clamp(0, 255).try_into()?;
        let g = typed!(args: Int).clamp(0, 255).try_into()?;
        let b = typed!(args: Int).clamp(0, 255).try_into()?;
        let a = typed!(args: Int? int => int.clamp(0, 255).try_into()?);

        self.0.set_draw_color((r, g, b, a.unwrap_or(255)));
        Ok(Value::default())
    }
    pub fn _scale(_: &mut Interpreter, mut args: Vec<Value>) -> Result<Value, Box<dyn Error>> {
        let Some(_self) = args.first().cloned() else {
            return Err(Box::new(UserObjectError::ExpectedSelf("null")));
        };
        args.remove(0);
        if let Value::UserObject(_self) = _self {
            let mut _self = _self.borrow_mut();
            _self.call_mut("scale", args)
        } else {
            Err(Box::new(UserObjectError::ExpectedSelf(_self.typ())))
        }
    }
    pub fn call_scale(&mut self, args: Vec<Value>) -> Result<Value, Box<dyn Error>> {
        let mut args = args.into_iter().enumerate();
        let scale_x = typed!(args: Float).clamp(0., f32::MAX.into()) as f32;
        let scale_y = typed!(args: Float).clamp(0., f32::MAX.into()) as f32;

        self.0.set_scale(scale_x, scale_y)?;
        Ok(Value::default())
    }
    pub fn _line(_: &mut Interpreter, mut args: Vec<Value>) -> Result<Value, Box<dyn Error>> {
        let Some(_self) = args.first().cloned() else {
            return Err(Box::new(UserObjectError::ExpectedSelf("null")));
        };
        args.remove(0);
        if let Value::UserObject(_self) = _self {
            let mut _self = _self.borrow_mut();
            _self.call_mut("line", args)
        } else {
            Err(Box::new(UserObjectError::ExpectedSelf(_self.typ())))
        }
    }
    pub fn call_line(&mut self, args: Vec<Value>) -> Result<Value, Box<dyn Error>> {
        let mut args = args.into_iter().enumerate();
        let start_x = option!(args:
            Int => int {
                int.clamp(i32::MIN.into(), i32::MAX.into()).try_into()?
            },
            Float => float {
                (float as i64).clamp(i32::MIN.into(), i32::MAX.into()).try_into()?
            }
        );
        let start_y = option!(args:
            Int => int {
                int.clamp(i32::MIN.into(), i32::MAX.into()).try_into()?
            },
            Float => float {
                (float as i64).clamp(i32::MIN.into(), i32::MAX.into()).try_into()?
            }
        );
        let end_x = option!(args:
            Int => int {
                int.clamp(i32::MIN.into(), i32::MAX.into()).try_into()?
            },
            Float => float {
                (float as i64).clamp(i32::MIN.into(), i32::MAX.into()).try_into()?
            }
        );
        let end_y = option!(args:
            Int => int {
                int.clamp(i32::MIN.into(), i32::MAX.into()).try_into()?
            },
            Float => float {
                (float as i64).clamp(i32::MIN.into(), i32::MAX.into()).try_into()?
            }
        );

        self.0.draw_line((start_x, start_y), (end_x, end_y))?;
        Ok(Value::default())
    }
    pub fn _point(_: &mut Interpreter, mut args: Vec<Value>) -> Result<Value, Box<dyn Error>> {
        let Some(_self) = args.first().cloned() else {
            return Err(Box::new(UserObjectError::ExpectedSelf("null")));
        };
        args.remove(0);
        if let Value::UserObject(_self) = _self {
            let mut _self = _self.borrow_mut();
            _self.call_mut("point", args)
        } else {
            Err(Box::new(UserObjectError::ExpectedSelf(_self.typ())))
        }
    }
    pub fn call_point(&mut self, args: Vec<Value>) -> Result<Value, Box<dyn Error>> {
        let mut args = args.into_iter().enumerate();
        let x = option!(args:
            Int => int {
                int.clamp(i32::MIN.into(), i32::MAX.into()).try_into()?
            },
            Float => float {
                (float as i64).clamp(i32::MIN.into(), i32::MAX.into()).try_into()?
            }
        );
        let y = option!(args:
            Int => int {
                int.clamp(i32::MIN.into(), i32::MAX.into()).try_into()?
            },
            Float => float {
                (float as i64).clamp(i32::MIN.into(), i32::MAX.into()).try_into()?
            }
        );

        self.0.draw_point((x, y))?;
        Ok(Value::default())
    }
    pub fn _rect(_: &mut Interpreter, mut args: Vec<Value>) -> Result<Value, Box<dyn Error>> {
        let Some(_self) = args.first().cloned() else {
            return Err(Box::new(UserObjectError::ExpectedSelf("null")));
        };
        args.remove(0);
        if let Value::UserObject(_self) = _self {
            let mut _self = _self.borrow_mut();
            _self.call_mut("rect", args)
        } else {
            Err(Box::new(UserObjectError::ExpectedSelf(_self.typ())))
        }
    }
    pub fn call_rect(&mut self, args: Vec<Value>) -> Result<Value, Box<dyn Error>> {
        let mut args = args.into_iter().enumerate();
        let x = option!(args:
            Int => int {
                int.clamp(i32::MIN.into(), i32::MAX.into()).try_into()?
            },
            Float => float {
                (float as i64).clamp(i32::MIN.into(), i32::MAX.into()).try_into()?
            }
        );
        let y = option!(args:
            Int => int {
                int.clamp(i32::MIN.into(), i32::MAX.into()).try_into()?
            },
            Float => float {
                (float as i64).clamp(i32::MIN.into(), i32::MAX.into()).try_into()?
            }
        );
        let width = option!(args:
            Int => int {
                int.clamp(u32::MIN.into(), u32::MAX.into()).try_into()?
            },
            Float => float {
                (float as i64).clamp(u32::MIN.into(), u32::MAX.into()).try_into()?
            }
        );
        let height = option!(args:
            Int => int {
                int.clamp(u32::MIN.into(), u32::MAX.into()).try_into()?
            },
            Float => float {
                (float as i64).clamp(u32::MIN.into(), u32::MAX.into()).try_into()?
            }
        );
        let fill = typed!(args: Bool?).unwrap_or_default();

        if fill {
            self.0.fill_rect(Rect::new(x, y, width, height))?;
        } else {
            self.0.draw_rect(Rect::new(x, y, width, height))?;
        }
        Ok(Value::default())
    }
}

pub struct EventPumpObject(EventPump);
impl UserObject for EventPumpObject {
    fn typ(&self) -> &'static str {
        "event-pump"
    }
    fn get(&self, key: &str) -> Option<Value> {
        match key {
            "pull" => Some(Value::Function(FunctionKind::UserFunction(Rc::new(
                Self::_pull,
            )))),
            _ => None,
        }
    }
    fn call_mut(&mut self, key: &str, _: Vec<Value>) -> Result<Value, Box<dyn Error>> {
        match key {
            "pull" => self.call_pull(),
            _ => Err(UserObjectError::CannotCallNull.into()),
        }
    }
}
impl EventPumpObject {
    pub fn _pull(_: &mut Interpreter, mut args: Vec<Value>) -> Result<Value, Box<dyn Error>> {
        let Some(_self) = args.first().cloned() else {
            return Err(Box::new(UserObjectError::ExpectedSelf("null")));
        };
        args.remove(0);
        if let Value::UserObject(_self) = _self {
            let mut _self = _self.borrow_mut();
            _self.call_mut("pull", args)
        } else {
            Err(Box::new(UserObjectError::ExpectedSelf(_self.typ())))
        }
    }
    pub fn call_pull(&mut self) -> Result<Value, Box<dyn Error>> {
        let event = self.0.poll_event();
        if let Some(event) = event {
            Ok(match event {
                Event::Quit { timestamp } => object! {
                    "kind" = "quit",
                    "timestamp" = timestamp
                },
                Event::AppTerminating { timestamp } => object! {
                    "kind" = "app_terminated",
                    "timestamp" = timestamp
                },
                Event::AppLowMemory { timestamp } => object! {
                    "kind" = "app_low_memory",
                    "timestamp" = timestamp
                },
                Event::AppWillEnterBackground { timestamp } => object! {
                    "kind" = "app_will_enter_background",
                    "timestamp" = timestamp
                },
                Event::AppDidEnterBackground { timestamp } => object! {
                    "kind" = "app_did_enter_background",
                    "timestamp" = timestamp
                },
                Event::AppWillEnterForeground { timestamp } => object! {
                    "kind" = "app_will_enter_foreground",
                    "timestamp" = timestamp
                },
                Event::AppDidEnterForeground { timestamp } => object! {
                    "kind" = "app_did_enter_foreground",
                    "timestamp" = timestamp
                },
                Event::Display {
                    timestamp,
                    display_index,
                    display_event,
                } => object! {
                    "kind" = "display",
                    "display_index" = display_index,
                    "display_event" = match display_event {
                        DisplayEvent::None => "none",
                        DisplayEvent::Connected => "connected",
                        DisplayEvent::Disconnected => "disconnected",
                        DisplayEvent::Orientation(orientation) => match orientation {
                            Orientation::Unknown => "unknown",
                            Orientation::Landscape => "landscape",
                            Orientation::LandscapeFlipped => "landscape_flipped",
                            Orientation::Portrait => "portrait",
                            Orientation::PortraitFlipped => "portrait_flipped",
                        },
                    },
                    "timestamp" = timestamp
                },
                Event::Window {
                    timestamp,
                    window_id,
                    win_event,
                } => object! {
                    "kind" = "window",
                    "window_id" = window_id,
                    "win_event" = match win_event {
                        WindowEvent::None => "none",
                        WindowEvent::Close => "close",
                        WindowEvent::Leave => "leave",
                        WindowEvent::Maximized => "maximized",
                        WindowEvent::Resized(_, _) => "resized",
                        WindowEvent::Hidden => "hidden",
                        WindowEvent::HitTest => "hit_test",
                        WindowEvent::FocusLost => "focus_lost",
                        WindowEvent::Enter => "enter",
                        WindowEvent::Minimized => "minimized",
                        WindowEvent::Moved(_, _) => "moved",
                        WindowEvent::Shown => "shown",
                        WindowEvent::DisplayChanged(_) => "display_changed",
                        WindowEvent::TakeFocus => "take_focus",
                        WindowEvent::FocusGained => "focus_gained",
                        WindowEvent::Restored => "restored",
                        WindowEvent::SizeChanged(_, _) => "size_changed",
                        WindowEvent::Exposed => "exposed",
                        WindowEvent::ICCProfChanged => "icc_prof_changed",
                    },
                    "width" = if let WindowEvent::Resized(width, _) | WindowEvent::SizeChanged(width, _) | WindowEvent::Moved(width, _) = win_event {
                        Value::Int(width as i64)
                    } else {
                        Value::default()
                    },
                    "height" = if let WindowEvent::Resized(_, height) | WindowEvent::SizeChanged(_, height) | WindowEvent::Moved(_, height) = win_event {
                        Value::Int(height as i64)
                    } else {
                        Value::default()
                    },
                    "timestamp" = timestamp
                },
                Event::KeyDown {
                    timestamp,
                    window_id,
                    keycode,
                    scancode,
                    keymod,
                    repeat,
                } => object! {
                    "kind" = "key_down",
                    "window_id" = window_id,
                    "keycode" = keycode.map(|code| Value::String(code.to_string().to_lowercase())).unwrap_or_default(),
                    "scancode" = scancode.map(|code| Value::String(code.to_string().to_lowercase())).unwrap_or_default(),
                    "keymod" = keymod.to_string().to_lowercase(),
                    "repeat" = repeat,
                    "timestamp" = timestamp
                },
                Event::KeyUp {
                    timestamp,
                    window_id,
                    keycode,
                    scancode,
                    keymod,
                    repeat,
                } => object! {
                    "kind" = "key_up",
                    "window_id" = window_id,
                    "keycode" = keycode.map(|code| Value::String(code.to_string().to_lowercase())).unwrap_or_default(),
                    "scancode" = scancode.map(|code| Value::String(code.to_string().to_lowercase())).unwrap_or_default(),
                    "keymod" = keymod.to_string().to_lowercase(),
                    "repeat" = repeat,
                    "timestamp" = timestamp
                },
                Event::TextEditing {
                    timestamp,
                    window_id,
                    text,
                    start,
                    length,
                } => object! {
                    "kind" = "text_editing",
                    "window_id" = window_id,
                    "text" = text,
                    "start" = start,
                    "length" = length,
                    "timestamp" = timestamp
                },
                Event::TextInput {
                    timestamp,
                    window_id,
                    text,
                } => object! {
                    "kind" = "text_input",
                    "window_id" = window_id,
                    "text" = text,
                    "timestamp" = timestamp
                },
                Event::MouseMotion {
                    timestamp,
                    window_id,
                    which,
                    mousestate: _,
                    x,
                    y,
                    xrel,
                    yrel,
                } => object! {
                    "kind" = "mouse_motion",
                    "window_id" = window_id,
                    "which" = which,
                    "x" = x,
                    "y" = y,
                    "xrel" = xrel,
                    "yrel" = yrel,
                    "timestamp" = timestamp
                },
                Event::MouseButtonDown {
                    timestamp,
                    window_id,
                    which,
                    mouse_btn,
                    clicks,
                    x,
                    y,
                } => object! {
                    "kind" = "mouse_button_down",
                    "window_id" = window_id,
                    "which" = which,
                    "mouse_btn" = mouse_btn as u8,
                    "clicks" = clicks,
                    "x" = x,
                    "y" = y,
                    "timestamp" = timestamp
                },
                Event::MouseButtonUp {
                    timestamp,
                    window_id,
                    which,
                    mouse_btn,
                    clicks,
                    x,
                    y,
                } => object! {
                    "kind" = "mouse_button_up",
                    "window_id" = window_id,
                    "which" = which,
                    "mouse_btn" = mouse_btn as u8,
                    "clicks" = clicks,
                    "x" = x,
                    "y" = y,
                    "timestamp" = timestamp
                },
                Event::MouseWheel {
                    timestamp,
                    window_id,
                    which,
                    x,
                    y,
                    direction,
                    precise_x,
                    precise_y,
                } => object! {
                    "kind" = "mouse_wheel",
                    "window_id" = window_id,
                    "which" = which,
                    "x" = x,
                    "y" = y,
                    "direction" = match direction {
                        MouseWheelDirection::Normal => 0,
                        MouseWheelDirection::Flipped => 1,
                        MouseWheelDirection::Unknown(v) => v,
                    },
                    "precise_x" = precise_x,
                    "precise_y" = precise_y,
                    "timestamp" = timestamp
                },
                // Event::JoyAxisMotion {
                //     timestamp,
                //     which,
                //     axis_idx,
                //     value,
                // } => todo!(),
                // Event::JoyBallMotion {
                //     timestamp,
                //     which,
                //     ball_idx,
                //     xrel,
                //     yrel,
                // } => todo!(),
                // Event::JoyHatMotion {
                //     timestamp,
                //     which,
                //     hat_idx,
                //     state,
                // } => todo!(),
                // Event::JoyButtonDown {
                //     timestamp,
                //     which,
                //     button_idx,
                // } => todo!(),
                // Event::JoyButtonUp {
                //     timestamp,
                //     which,
                //     button_idx,
                // } => todo!(),
                // Event::JoyDeviceAdded { timestamp, which } => todo!(),
                // Event::JoyDeviceRemoved { timestamp, which } => todo!(),
                // Event::ControllerAxisMotion {
                //     timestamp,
                //     which,
                //     axis,
                //     value,
                // } => todo!(),
                // Event::ControllerButtonDown {
                //     timestamp,
                //     which,
                //     button,
                // } => todo!(),
                // Event::ControllerButtonUp {
                //     timestamp,
                //     which,
                //     button,
                // } => todo!(),
                // Event::ControllerDeviceAdded { timestamp, which } => todo!(),
                // Event::ControllerDeviceRemoved { timestamp, which } => todo!(),
                // Event::ControllerDeviceRemapped { timestamp, which } => todo!(),
                // Event::ControllerTouchpadDown {
                //     timestamp,
                //     which,
                //     touchpad,
                //     finger,
                //     x,
                //     y,
                //     pressure,
                // } => todo!(),
                // Event::ControllerTouchpadMotion {
                //     timestamp,
                //     which,
                //     touchpad,
                //     finger,
                //     x,
                //     y,
                //     pressure,
                // } => todo!(),
                // Event::ControllerTouchpadUp {
                //     timestamp,
                //     which,
                //     touchpad,
                //     finger,
                //     x,
                //     y,
                //     pressure,
                // } => todo!(),
                // Event::FingerDown {
                //     timestamp,
                //     touch_id,
                //     finger_id,
                //     x,
                //     y,
                //     dx,
                //     dy,
                //     pressure,
                // } => todo!(),
                // Event::FingerUp {
                //     timestamp,
                //     touch_id,
                //     finger_id,
                //     x,
                //     y,
                //     dx,
                //     dy,
                //     pressure,
                // } => todo!(),
                // Event::FingerMotion {
                //     timestamp,
                //     touch_id,
                //     finger_id,
                //     x,
                //     y,
                //     dx,
                //     dy,
                //     pressure,
                // } => todo!(),
                // Event::DollarGesture {
                //     timestamp,
                //     touch_id,
                //     gesture_id,
                //     num_fingers,
                //     error,
                //     x,
                //     y,
                // } => todo!(),
                // Event::DollarRecord {
                //     timestamp,
                //     touch_id,
                //     gesture_id,
                //     num_fingers,
                //     error,
                //     x,
                //     y,
                // } => todo!(),
                // Event::MultiGesture {
                //     timestamp,
                //     touch_id,
                //     d_theta,
                //     d_dist,
                //     x,
                //     y,
                //     num_fingers,
                // } => todo!(),
                // Event::ClipboardUpdate { timestamp } => todo!(),
                // Event::DropFile {
                //     timestamp,
                //     window_id,
                //     filename,
                // } => todo!(),
                // Event::DropText {
                //     timestamp,
                //     window_id,
                //     filename,
                // } => todo!(),
                // Event::DropBegin {
                //     timestamp,
                //     window_id,
                // } => todo!(),
                // Event::DropComplete {
                //     timestamp,
                //     window_id,
                // } => todo!(),
                // Event::AudioDeviceAdded {
                //     timestamp,
                //     which,
                //     iscapture,
                // } => todo!(),
                // Event::AudioDeviceRemoved {
                //     timestamp,
                //     which,
                //     iscapture,
                // } => todo!(),
                // Event::RenderTargetsReset { timestamp } => todo!(),
                // Event::RenderDeviceReset { timestamp } => todo!(),
                // Event::User {
                //     timestamp,
                //     window_id,
                //     type_,
                //     code,
                //     data1,
                //     data2,
                // } => todo!(),
                Event::Unknown { timestamp, type_ } => object! {
                    "timestamp" = timestamp,
                    "type" = type_
                },
                _ => Value::default()
            })
        } else {
            Ok(Value::default())
        }
    }
}
