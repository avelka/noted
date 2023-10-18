use leptos::*;
const SEK2:&str =  "Sek 2";
const SEK1:&str =  "Sek 1";
const LANG:&str =  "Sprachen";

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
fn s2f(s: &str) -> f32 {
    s.parse::<f32>().unwrap()
}

fn custom_scale(val: i32, points: i32) -> String {
    let s = points as f32;
    let v = val as f32;
    let res = format!("{:.1}", ((v / 100.0) * s * 2.0).round() / 2.0);
    let rounded = res.strip_suffix(".0");
    match rounded {
        Some(r) => r.to_string(),
        None => res.to_string(),
    }
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
fn NoteItem(id: i32, note: Note, set_notes: WriteSignal<Vec<(i32, Note)>>, points: ReadSignal<i32>  ) -> impl IntoView {
    let (value, set_value) = note.value;
    let (ratio, set_ratio) = note.ratio;

    let point_match = move |scale| custom_scale(scale, points.get());
    let (name, set_name) = note.name;

    let (local_scale, set_local_scale) = note.local_scale;
 
    let scale_labels = move || get_labels_array(local_scale().as_str()); 
    let threshold_array = move || get_threshold_array(local_scale().as_str()); 
    let note = move || {
        let v = value();
        let above = threshold_array().iter().position(|&r| r >= v).unwrap_or(16);
        let label = scale_labels()[above]; 
        return label;
    };

    view! {
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
                            .map(|s| { view! { 
                                <option value={s} selected={local_scale() == s.to_string()} >{s}</option> 
                            } })
                        .collect_view()
                    }
                </select>
            </div>
            <div class="form-control">
                <label for="note">"Note"</label>
                <select id="note" name="note" 
                    on:change=move |ev| {
                       let label = event_target_value(&ev);
                       let labelIndex = scale_labels().iter().position(|&r| r == label).unwrap();
                       let minNote = threshold_array()[labelIndex];
                       set_value(minNote);
                    }
                    prop:value=note>
                    {move ||
                        scale_labels().map(|el| { view! { 
                            <option value={el} selected={move || note().to_string() == el} >{el}</option> 
                        }})
                        .collect_view()
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
                <label for="note_point">"Point (#)"</label>
                <input class="no-input-ui" name="note_point" type="number" input-mode="numeric"
                    min="0"
                    max={move || points.get()}
                    step="0.5"

                    on:input=move |ev| {
                       let val_from_point = (s2f(&event_target_value(&ev)) / (points.get() as f32) * 100.0).round() as i32;
                        
                       set_value(val_from_point);
                    }
                    prop:value=move || point_match(value())
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
fn NoteList() -> impl IntoView {
 
    let mut next_note_id = 0;
    let (global_scale, set_global_scale) = create_signal(SEK1.to_string());
    let (notes, set_notes) = create_signal(vec![]);
    let global_scale_labels = move || get_labels_array(global_scale().as_str()); 
    let global_threshold_array = move || get_threshold_array(global_scale().as_str());

    let (points, set_points) = create_signal(100);
    let point_match = move |scale| custom_scale(scale, points.get());
    let add_note = move |_| {
        let note = Note {
            value: create_signal(0),
            local_scale: create_signal(SEK1.to_string()),
            ratio: create_signal(1),
            name: create_signal("Note".to_string()),
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
            return (v * r, r);
        }).reduce(|(at, ar), (bt, br)| (at + bt, ar + br)).unwrap_or((0.00, 1.00)); 

        return (tt / tr).round() as i32;
    }; 

    let global_note = move || {
        let v = get_total();
        let above = global_threshold_array().iter().position(|&r| r >= v).unwrap_or(16);
        let label = global_scale_labels()[above]; 
        return label;

    };

    view! { 
        <main>
              <div class="form-control standalone">
                <label for="points">"Points"</label>
                <input id="points" type="number" name="points" prop:value=points on:change=move |ev| {
                    set_points(s2n(&event_target_value(&ev)))
                }/>
            </div>

            <div class="table-container"><table class="scale-table">
                <caption>{move || global_scale()}</caption>
                <colgroup>
                    {move || global_scale_labels().map(|_| { view! { <col/> } })}
                </colgroup>
                <tr>
                    <th>"Note"</th>
                    {move || global_scale_labels().map(|s| { view! { <td>{s}</td> } })}
                </tr>
                <tr>
                    <th>"% ≥"</th>
                    {move || global_threshold_array().map(|s| { view! { <td>{s}</td> } })}
                </tr>
                <Show when=move || { points.get() != 100 }> 
                <tr>
                    <th>"Points"</th>
                    {move || global_threshold_array().map(|s| { view! { <td>{point_match(s)}</td> } })}
                </tr>
                </Show>
            </table>
            </div>
            <ul class="note-list">
                <For
                    each=notes
                    key=|note| note.0
                    children=move |(id, note)| {
                        view! { 
                            <NoteItem id=id note=note set_notes=set_notes points=points />
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
                            .map(|s| { view! { 
                                <option value={s} selected={global_scale() == s.to_string()} >{s}</option> 
                            } })
                        .collect_view()
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
fn App() -> impl IntoView {
    view! {
    <div class="app">
        <header>
            <img src="/noted.svg" alt="Noted" width="100" height="100" />
            <h1 class="sr-only">"Noted"</h1>
        </header>
        <NoteList/>
    </div>
}}

fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}
