use leptos::*;
const SEK2:&str =  "Sek 2";
const SEK1:&str =  "Sek 1";
const LANG:&str =  "Lang";

static SCALES: [&str; 3] = [SEK2, SEK1, LANG];

static SEK2_LABELS: [&'static str; 16] = ["0", "1", "2", "3","4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15"];
static SEK2_THRESHOLD: [i32; 16] = [0, 20, 27, 33, 40, 45, 50, 55, 60 , 65 ,70 ,75, 80, 85,90, 95];

static LANG_LABELS: [&'static str; 16] = ["6", "6+", "5-", "5", "5+", "4-", "4", "4+", "3-", "3", "3+", "2-", "2", "2+", "1-", "1"];
static LANG_THRESHOLD: [i32; 16] = [0, 23, 25, 27, 47, 49, 51, 62, 64 , 66 ,76 ,78, 80, 90, 92, 94];

static SEK1_LABELS: [&'static str; 16] = ["6", "5-", "5", "5+", "4-", "4", "4+", "3-", "3", "3+", "2-", "2", "2+", "1-", "1", "1+"];
static SEK1_THRESHOLD: [i32; 16] = [0, 20, 29, 37, 45, 50, 55, 60, 65, 70, 75, 80, 85, 90, 94, 98];


#[derive(Clone, Copy)]
struct Note  {
    value: (ReadSignal<i32>, WriteSignal<i32>),
    ratio: (ReadSignal<i32>, WriteSignal<i32>),
    name: (ReadSignal<String>, WriteSignal<String>),
    local_scale: (ReadSignal<String>, WriteSignal<String>),
}

fn s2n(s: &str) -> i32 {
    s.parse::<i32>().unwrap()
}

fn get_threshold_array(scale: &str) -> [i32; 16] {
    match scale {
        SEK2 => SEK2_THRESHOLD,
        LANG=> LANG_THRESHOLD, 
        SEK1=> SEK1_THRESHOLD,
        _ => panic!("No scale found"),
    }
}

fn get_labels_array(scale: &str) -> [&'static str; 16] {
    match scale {
        SEK2 => SEK2_LABELS, 
        LANG => LANG_LABELS,
        SEK1=> SEK1_LABELS,
        _ => panic!("No scale found"),
    }
}

#[component]
fn NoteItem(cx: Scope, id: i32, note: Note, set_notes: WriteSignal<Vec<(i32, Note)>> ) -> impl IntoView {
    let (value, set_value) = note.value;
    let (ratio, set_ratio) = note.ratio;
    let (name, set_name) = note.name;

    let (local_scale, set_local_scale) = note.local_scale;
 
    let scale_labels = move || get_labels_array(local_scale().as_str()); 
    let threshold_array = move || get_threshold_array(local_scale().as_str()); 
   let note = move || {
        let mut i = 0;
        let mut note = "";
        let labels = scale_labels();
        while i < labels.len() {
            if value() < threshold_array()[i] {
                note = labels[i - 1];
                break;
            }
            note = labels[i];
            i += 1;
        }
        note
    };
 

    view! { cx,
        <li class="note-item">
        <form>
            <div class="form-control">
                <label for="name">"Name"</label>
                <input name="name" type="text"
                    on:input=move |ev| {
                       set_name(event_target_value(&ev));
                    }
                    prop:value=name
                />
            </div>
            <div class="form-control">
                <label for="scale">"Scale"</label>
                <select id="scale" name="scale" on:change=move |ev| { 
                    set_local_scale(event_target_value(&ev)) 
                }>
                    {
                        SCALES
                            .into_iter()
                            .map(|s| { view! { cx, 
                                <option value={s} selected={local_scale() == s.to_string()} >{s}</option> 
                            } })
                        .collect_view(cx)
                    }
                </select>
            </div>
            <div class="form-control">
                <label for="note">"Note"</label>
                <select id="note" name="note" 
                    on:change=move |ev| {
                       let scale = event_target_value(&ev);
                       set_value(threshold_array()[s2n(&scale) as usize]);
                    }
                    prop:value=note>
                    {move ||
                        scale_labels().into_iter().enumerate().map(|(i, el)| { view! { cx, 
                                <option value={i} selected={move || note().to_string() == i.to_string()} >{el}</option> 
                            } })
                        .collect_view(cx)
                    }
                </select>
            </div>
            <div class="form-control">
                <label for="note_value">"Note (%)"</label>
                <input name="note_value" type="number" 
                    max="100"
                    min="0"
                    on:input=move |ev| {
                       set_value(s2n(&event_target_value(&ev)));
                    }
                    prop:value=value
                />
            </div>
            <div class="form-control">
                <label for="note_ratio">"Ratio"</label>
                <input id="note_ratio" name="note_ratio" type="number" 
                    on:input=move |ev| {
                       set_ratio(s2n(&event_target_value(&ev)));
                    }
                    min="1"
                    prop:value=ratio
                />
            </div>
        </form>

        <button on:click= move |_| {
            set_notes.update(move |notes| {
                notes.retain(|(i, _)| *i != id)
            })
        } >"X"</button>
        </li>
    }
}

#[component]
fn NoteList(
    cx: Scope,
) -> impl IntoView {
 
    let mut next_note_id = 0;
    let (global_scale, set_global_scale) = create_signal(cx, SEK2.to_string());
    let (notes, set_notes) = create_signal(cx, vec![]);
    let global_scale_labels = move || get_labels_array(global_scale().as_str()); 
    let global_threshold_array = move || get_threshold_array(global_scale().as_str()); 
    let add_note = move |_| {
        let note = Note {
            value: create_signal(cx, 0),
            local_scale: create_signal(cx, "Sek 2".to_string()),
            ratio: create_signal(cx, 1),
            name: create_signal(cx, "Note".to_string()),
        };
        set_notes.update(move |notes| {
            notes.push((next_note_id, note))
        });
        next_note_id += 1;
    };
    
    let get_total = move || { 
        let (tt, tr) = notes().iter().map(move |&(_, note)| {
            let (value, _) = note.value;
            let (ratio, _) = note.ratio;
            let v = value() as f32;
            let r = ratio() as f32;
            let t = v * r;
            return (t, r);
        }).reduce(|(at, ar), (bt, br)| (at + bt, ar + br)).unwrap_or((0.00, 1.00)); 

        
        return tt / tr;
    }; 
    let global_note = move || {
        let mut i = 0;
        let mut note = "";
        let labels = global_scale_labels();
        while i < labels.len() {

            if get_total() < global_threshold_array()[i] as f32  {
                note = labels[i - 1];
                break;
            }
            note = labels[i];
            i += 1;
        }
        note
    };
    view! { cx,
        <main>
            <table class="scale-table">
                <caption>{move || global_scale()}</caption>
                <colgroup>
                    {move || global_scale_labels().map(|_| { view! { cx, <col/> } })}
                </colgroup>
                <tr>
                    <th>"Note"</th>
                    {move || global_scale_labels().map(|s| { view! { cx, <td>{s}</td> } })}
                </tr>
                <tr>
                    <th>"% ≥"</th>

                    {move || global_threshold_array().map(|s| { view! { cx, <td>{s}</td> } })}
                </tr>
            </table>
            <ul class="note-list">
                <For
                    each=notes
                    key=|note| note.0
                    view=move |cx, (id, note)| {
                        view! { cx,
                            <NoteItem id=id note=note set_notes=set_notes/>
                        }
                    }
                />
            </ul>
            <button on:click=add_note>
                "Add"
            </button>

          </main>  
            <footer class="total">
                 <div class="form-control">
                    <label class="sr-only" for="scale">"Scale"</label>
                    <select id="scale" name="scale" on:change=move |ev| { 
                        set_global_scale(event_target_value(&ev)) 
                    }>
                    {
                        SCALES
                            .into_iter()
                            .map(|s| { view! { cx, 
                                <option value={s} selected={global_scale() == s.to_string()} >{s}</option> 
                            } })
                        .collect_view(cx)
                    }
                    </select>
                </div>
                <div class="total">
                    <strong>{move || global_note()}</strong> " | "
                    <small>{move || get_total()}"%"</small>
                </div>
                <div/>
            </footer>
    }
}


#[component]
fn App(cx: Scope) -> impl IntoView {
    view! { cx,
    <div class="app">
        <header>
            <h1>"🦀 🎓 👩‍🎓"</h1>
        </header>
        <NoteList/>
    </div>
}}

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}
