use std::sync::Arc;
use crate::utils::*;
use vizia::*;
use vst::host::Host;
use vst::plugin::HostCallback;
use vst::plugin::PluginParameters;
use crate::editor::EditorState;
use crate::parameter::*;
use crate::FilterParameters;

const STYLE: &str = r#"
    label {
        font-size: 20;
        color: #C2C2C2;
    }
    knob {
        width: 70px;
        height: 70px;
    }
    
    knob .track {
        background-color: #ffb74d;
    }
"#;

#[derive(Lens)]
pub struct Params {
    params: Arc<FilterParameters>,
    host: Option<HostCallback>,
}

#[derive(Debug)]
pub enum ParamChangeEvent {
    _SetGain(f32),
    AllParams(i32, f32),
}

impl Model for Params {
    fn event(&mut self, _cx: &mut Context, event: &mut Event) {
        if let Some(param_change_event) = event.message.downcast() {
            match param_change_event {
                ParamChangeEvent::_SetGain(_new_gain) => {
                    
                }
                ParamChangeEvent::AllParams(parameter_index, new_value,) => {
                    // host needs to know that the parameter should/has changed
                    if let Some(host) = self.host {
                        host.begin_edit(*parameter_index);
                        host.automate(*parameter_index, *new_value);
                        host.end_edit(*parameter_index);
                    }
                    // set_parameter is on the PluginParameters trait
                    else {
                        self.params.set_parameter(*parameter_index, *new_value);
                    }
                }
            }
        }
    }
}

pub fn plugin_gui(cx: &mut Context, state: Arc<EditorState> ) {
    cx.add_theme(STYLE);

    Params {
        params: state.params.clone(),
        host: state.host,
    }.build(cx);

    HStack::new(cx, |cx| {
        VStack::new(cx, |cx|{
            Binding::new(cx, Params::params, move |cx, params|{
                let param_index = 0;
                Label::new(cx, &params.get(cx).get_parameter_name(param_index));
                // let param_ref = params.get(cx);
                // Knob::new(cx, map.clone(), params.osc_p[0].volume.get_normalized_default()).on_changing(cx, |knob, cx|{
                Knob::new(cx, params.get(cx)._get_parameter_default(param_index), params.get(cx).get_parameter(param_index), false).on_changing(cx, |knob, cx,|{
                    cx.emit(ParamChangeEvent::AllParams(0, knob.normalized_value))
                });
                Label::new(cx, &params.get(cx).get_parameter_text(param_index));
            });
        }).child_space(Stretch(1.0)).row_between(Pixels(10.0));
    
        VStack::new(cx, |cx|{
            Binding::new(cx, Params::params, move |cx, params|{
                let param_index = 1;
                Label::new(cx, &params.get(cx).get_parameter_name(param_index));
                Knob::new(cx, params.get(cx)._get_parameter_default(param_index), params.get(cx).get_parameter(param_index), false).on_changing(cx, |knob, cx,|{
                    cx.emit(ParamChangeEvent::AllParams(1, knob.normalized_value))
                });
                Label::new(cx, &params.get(cx).get_parameter_text(param_index));
            });
        }).child_space(Stretch(1.0)).row_between(Pixels(10.0));

        VStack::new(cx, |cx|{
            Binding::new(cx, Params::params, move |cx, params|{
                let param_index = 2;
                Label::new(cx, &params.get(cx).get_parameter_name(param_index));
                Knob::new(cx, params.get(cx)._get_parameter_default(param_index), params.get(cx).get_parameter(param_index), false).on_changing(cx, |knob, cx,|{
                    // cx.emit(ParamChangeEvent::SetGain(knob.normalized_value));
                    cx.emit(ParamChangeEvent::AllParams(2, knob.normalized_value))
                });
                Label::new(cx, &params.get(cx).get_parameter_text(param_index));
            });
            
        }).child_space(Stretch(1.0)).row_between(Pixels(10.0));
        
        VStack::new(cx, |cx|{
            Binding::new(cx, Params::params, |cx, params|{
                let ft = params.get(cx).filter_type.get();
                Label::new(cx, if ft == 0 {"Filter Mode"} else {"Slope"});
                let val = if ft == 0 {params.get(cx).mode.get_normalized()} else {params.get(cx).slope.get_normalized() };
                let default = if ft == 0 {params.get(cx).mode.get_normalized_default()} else {params.get(cx).slope.get_normalized_default() };
                Knob::new(cx, default, val, false).on_changing(cx, move |knob, cx|{
                    cx.emit(ParamChangeEvent::AllParams(if ft == 0 {4} else {5}, knob.normalized_value))
                });
                Binding::new(cx, Params::params, move |cx, params|{
                    let ft = params.get(cx).filter_type.get();

                    Label::new(cx, &params.get(cx).get_parameter_text(if ft == 0 {4} else {5}));
        
                });

            })
        }).child_space(Stretch(1.0)).row_between(Pixels(10.0));
        // VStack::new(cx, |cx|{
        //     Label::new(cx, "Filter circuit");
        //     let map = GenericMap::new(0.0, 1.0, ValueScaling::Linear, DisplayDecimals::Two, None);
        //     Knob::new(cx, map.clone(), 0.5).on_changing(cx, |knob, cx|{
    
        //         // cx.emit(ParamChangeEvent::SetGain(knob.normalized_value));
        //         cx.emit(ParamChangeEvent::AllParams(3, knob.normalized_value))
        //     });
        //     Binding::new(cx, Params::params, move |cx, params|{
        //         let ft = params.get(cx).filter_type.get();

        //         Label::new(cx, if ft == 0 {"SVF"} else {"Ladder"});
    
        //     });
        // }).child_space(Stretch(1.0)).row_between(Pixels(10.0));
    }).background_color(Color::rgb(25, 25, 25)).child_space(Stretch(1.0)).row_between(Pixels(0.0));
    
    


}