use crate::consts::*;

/// An action that a button can be mapped to.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Action {
    /// Button has single action/command
    Single(Command),
    /// Button executes a procedure.
    Proc(ProcIdx),
    /// No operation - Action is undefined.
    Noop,
}

/// Mapping from (button (input), trigger, layer) into action.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Binding {
    /// Input ID
    pub idx: InIdx,
    /// What layer does it apply to.
    pub layer: LayerIdx,
    /// What is trigger type.
    pub trigger: Trigger,
    /// What action to execute.
    pub action: Action,
}

impl Binding {
    pub fn short(idx: InIdx, layer: LayerIdx, out_idx: OutIdx) -> Self {
        Self {
            idx,
            layer,
            action: Action::Single(Command::ToggleOutput(out_idx)),
            trigger: Trigger::ShortClick,
        }
    }

    pub fn long(idx: InIdx, layer: LayerIdx, out_idx: OutIdx) -> Self {
        Self {
            idx,
            layer,
            action: Action::Single(Command::ToggleOutput(out_idx)),
            trigger: Trigger::LongClick,
        }
    }
}

impl Default for Binding {
    fn default() -> Self {
        Self {
            idx: 0,
            layer: 0,
            action: Action::Noop,
            trigger: Trigger::ShortClick,
        }
    }
}

/// Keeps bindings and finds the valid ones.
pub struct BindingList<const N: usize> {
    bindings: [Binding; N],
    added: usize,
}

impl<const N: usize> Default for BindingList<N> {
    fn default() -> Self {
        Self {
            bindings: [Binding::default(); N],
            added: 0,
        }
    }
}

impl<const N: usize> BindingList<N> {
    pub fn new() -> Self {
        Self::default()
    }

    /// Clear defined bindings
    pub fn clear(&mut self) {
        for i in 0..N {
            self.bindings[i] = Binding::default();
        }
        self.added = 0;
    }

    /// Find first index of bindings for given input.
    fn find_first_idx(&self, input_idx: InIdx) -> Option<usize> {
        if let Ok(mut idx) = self.bindings[0..self.added].binary_search_by_key(&input_idx, |b| b.idx) {
            // Something was found, but no guarantee it's the first one.
            while idx > 0 {
                if self.bindings[idx - 1].idx != input_idx {
                    break;
                }
                idx -= 1;
            }
            Some(idx)
        } else {
            None
        }
    }

    /// Find index of the first binding that match the filters.
    fn find_idx_filtered(
        &self,
        input_idx: InIdx,
        layer: Option<LayerIdx>,
        trigger: Option<Trigger>,
    ) -> Option<usize> {
        let first_idx = self.find_first_idx(input_idx)?;
        for i in first_idx..self.added {
            let binding = &self.bindings[i];
            if binding.idx != input_idx {
                return None;
            }
            if let Some(layer) = layer {
                if layer != binding.layer {
                    continue;
                }
            }
            if let Some(trigger) = trigger {
                if trigger != binding.trigger {
                    continue;
                }
            }
            return Some(i);
        }
        None
    }

    /// Find first matching binding if any. Lowest layer is returned.
    pub fn filter(
        &self,
        input_idx: InIdx,
        layer: Option<LayerIdx>,
        trigger: Option<Trigger>,
    ) -> Option<&Binding> {
        self.find_idx_filtered(input_idx, layer, trigger)
            .map(|idx| &self.bindings[idx])
    }

    /// Bind input (overwrite based on input idx and layer or add new binding)
    pub fn bind(&mut self, binding: Binding) {
        assert!(binding.idx != 0);

        if let Some(idx) =
            self.find_idx_filtered(binding.idx, Some(binding.layer), Some(binding.trigger))
        {
            // Overwrite this index.
            self.bindings[idx] = binding;
        } else {
            assert!(self.added < N, "Too many bindings added");
            self.bindings[self.added] = binding;
            self.added += 1;
            // Sort by layer to return lowest layer on .filter() without defined
            // precise layer.
            self.bindings[0..self.added].sort_by_key(|b| (b.idx, b.layer));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_and_finds() {
        let mut blst: BindingList<30> = BindingList::new();
        assert_eq!(blst.added, 0);
        for binding in [
            Binding::short(1 /*idx*/, 0 /*layer*/, 1 /*output*/),
            Binding::short(2, 0, 2),
            Binding::short(3, 0, 3), /* Will overwrite to 4 */
            Binding::long(1, 1, 1),
            Binding::long(2, 1, 2),
            Binding::long(3, 1, 3),
            Binding::long(1, 0, 1), /* Overwrite to 2 */
            Binding::long(2, 0, 2),
            Binding::long(3, 0, 3),
        ] {
            blst.bind(binding);
        }
        assert_eq!(blst.added, 9);

        // Overwrite some
        blst.bind(Binding::short(3, 0, 4));
        blst.bind(Binding::long(1, 0, 2));

        // Add a new one, and ovewrite it
        blst.bind(Binding::short(3, 2, 5));
        blst.bind(Binding::short(3, 2, 6));

        assert_eq!(blst.added, 10);

        let binding = blst.filter(2, None, None).unwrap();
        assert_eq!(binding.idx, 2);
        assert_eq!(binding.layer, 0);
        assert!(binding.trigger == Trigger::ShortClick || binding.trigger == Trigger::LongClick);

        let binding = blst.filter(2, Some(1), Some(Trigger::LongClick)).unwrap();
        assert_eq!(binding.idx, 2);
        assert_eq!(binding.layer, 1);
        assert!(binding.trigger == Trigger::LongClick);

        assert!(blst.filter(2, Some(1), Some(Trigger::ShortClick)).is_none());

        /* Overwritten ones */
        assert_eq!(blst.filter(3, Some(2), Some(Trigger::ShortClick)).unwrap().action,
                   Action::Single(Command::ToggleOutput(6)));
        assert_eq!(blst.filter(1, Some(0), Some(Trigger::LongClick)).unwrap().action,
                   Action::Single(Command::ToggleOutput(2)));

        for (i, entry) in blst.bindings.iter().enumerate() {
            println!("{} {:?}", i, entry);
        }

        blst.clear();
        assert_eq!(blst.added, 0);
    }
}
