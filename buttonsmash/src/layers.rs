use crate::consts::{InIdx, LayerIdx, MAX_LAYER_STACK};

pub struct Layers {
    /// Currently active layer.
    pub current: LayerIdx,
    /// Mapping between layers and buttons that activated them. Used to
    /// deactivate layers in a correct order.
    stack: [Option<(InIdx, LayerIdx)>; MAX_LAYER_STACK],
}

impl Layers {
    pub fn new() -> Self {
        Self {
            current: 0,
            stack: [None; MAX_LAYER_STACK],
        }
    }

    /// Reset state to layer 0 with no stack.
    pub fn reset(&mut self) {
        self.current = 0;
        for entry in self.stack.iter_mut() {
            *entry = None;
        }
    }

    /// Activate layer and store slot entry.
    pub fn activate(&mut self, in_idx: InIdx, layer: LayerIdx) {
        let slot_idx = self.find_slot();
        self.stack[slot_idx] = Some((in_idx, layer));
        self.current = layer;
    }

    /// Scan stack for activations using this input key and if one is found -
    /// deactivate it and return true. Otherwise return false.
    pub fn maybe_deactivate(&mut self, in_idx: InIdx) -> bool {
        let mut found: Option<usize> = None;
        for (slot_idx, entry) in self.stack.iter().enumerate() {
            if let Some((stack_in_idx, _stack_layer_idx)) = entry {
                if *stack_in_idx == in_idx {
                    // Continue search: Find latest matching entry.
                    found = Some(slot_idx);
                }
            } else {
                // Reached the end.
                break;
            }
        }
        if let Some(slot_idx) = found {
            self.drop_slot(slot_idx);
            if slot_idx == 0 {
                self.current = 0;
            } else {
                // Return back to previous layer
                let previous_layer_idx = self.stack[slot_idx - 1].expect("This must be Some").1;
                self.current = previous_layer_idx;
            }
            true
        } else {
            false
        }
    }

    /// Find and return index to a first free slot.
    fn find_slot(&self) -> usize {
        for i in 0..MAX_LAYER_STACK {
            if self.stack[i].is_none() {
                return i;
            }
        }
        panic!("Layer depth reached.");
    }

    /// Drop slot of given index and shift the rest (if any) to fill the gap.
    fn drop_slot(&mut self, slot_idx: usize) {
        assert!(self.stack[slot_idx].is_some());
        self.stack[slot_idx] = None;
        for i in slot_idx + 1..MAX_LAYER_STACK {
            if self.stack[i].is_none() {
                return;
            }
            self.stack[i - 1] = self.stack[i];
        }
    }

}
