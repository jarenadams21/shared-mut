// Element in space
struct Element {
   field1: String,
   field2: Number, 
}

// One cell of a human
struct HumanCell {

}

// Act as boosters that apply side effects to the search in different ways based on sys arch.
struct Electrolyte {

}

// Elements of the human body composition 
struct BodyComposition {
    hydrogen: Element,
    oxygen: Element, 
    carbon: Element,
    calcium: Element,
    phosphorus: Element,
}

// Intermediate state that transfers from human states in the frontend 
// to system health states, side effects, and more to enhance search.
struct HumanToSystemHealthIntermediate {
    body_water_to_convert_to_system_health: Unknown,
    body_composition: BodyComposition,
    electrolytes: Electrolyte,
    cells: Vec<HumanCell>,
    human_info_hash: Hash,
}



struct Runtime {

}

struct ProcessorArch {

}