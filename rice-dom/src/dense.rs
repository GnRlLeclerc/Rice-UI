//! Dense storage using bitmask

/// A dense map implemented using a bitmask.
/// Can hold up to 64 properties.
///
/// Reasons:
/// - O(1) access without hashing
/// - Dense storage (very small when empty)
///
/// Issues:
/// - O(n) insert / remove (but should not happen often)
pub struct DenseMap<V> {
    mask: u64,
    values: Vec<V>,
}

impl<V> DenseMap<V> {
    pub fn new() -> Self {
        Self {
            mask: 0,
            values: Vec::new(),
        }
    }

    /// Set or update a value
    pub fn set(&mut self, key: u8, value: V) {
        let bit = 1 << (key as u8);
        if self.mask & bit == 0 {
            // not set yet
            self.values.push(value);
            self.mask |= bit;
        } else {
            // already set, update value
            let index = self.index(key);
            self.values[index] = value;
        }
    }

    /// Get a value
    pub fn get(&self, key: u8) -> Option<&V> {
        if self.mask & (1 << (key as u8)) == 0 {
            None
        } else {
            let index = self.index(key);
            Some(&self.values[index])
        }
    }

    /// Remove a value
    pub fn remove(&mut self, key: u8) -> Option<V> {
        let bit = 1 << (key as u8);
        if self.mask & bit == 0 {
            None
        } else {
            let index = self.index(key);
            self.mask &= !bit;
            Some(self.values.remove(index))
        }
    }

    /// Get an iterator over set keys and values
    pub fn iter(&self) -> DenseMapIter<'_, V> {
        DenseMapIter {
            index: 0,
            mask: self.mask,
            map: &self.values,
        }
    }

    /// Check if the map is empty
    pub fn is_empty(&self) -> bool {
        self.mask == 0
    }

    /// Get the index of a key's value in the dense vector
    /// Assumes the key is present
    fn index(&self, key: u8) -> usize {
        (self.mask & ((1 << (key)) - 1)).count_ones() as usize
    }
}

/// Iterator over set keys and values in the dense map
pub struct DenseMapIter<'a, V> {
    index: usize,
    mask: u64,
    map: &'a [V],
}

impl<'a, V> Iterator for DenseMapIter<'a, V> {
    type Item = (u8, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        if self.mask == 0 {
            return None;
        }
        // Get the key index
        let tz = self.mask.trailing_zeros();
        self.mask ^= 1 << tz;

        // Get the value index
        let value = &self.map[self.index];
        self.index += 1;

        Some((tz as u8, value))
    }
}
