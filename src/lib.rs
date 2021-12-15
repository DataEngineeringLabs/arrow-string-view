mod string_view;

use arrow2::array::Array;
use arrow2::array::Utf8Array;
use arrow2::buffer::Buffer;

#[derive(Debug)]
pub struct StringView {
    pub values: Buffer<u8>,
    pub offsets: Buffer<u64>,
    pub lengths: Buffer<u64>,
}

impl StringView {
    pub fn from_iterator<T: AsRef<str>, I: Iterator<Item = T>>(iter: I) -> Self {
        let mut values = vec![];
        let mut offsets = Vec::with_capacity(iter.size_hint().0);
        let mut lengths = Vec::with_capacity(iter.size_hint().0);
        let mut length = 0u64;
        for item in iter {
            let item = item.as_ref();
            values.extend_from_slice(item.as_bytes());

            lengths.push(item.len() as u64);
            length += item.len() as u64;
            offsets.push(length);
        }

        StringView {
            values: Buffer::from_vec(values),
            offsets: Buffer::from_vec(offsets),
            lengths: Buffer::from_vec(lengths),
        }
    }
}

pub fn take_view(view: &StringView, indices: &[u64]) -> StringView {
    let mut offsets = Vec::with_capacity(indices.len());
    let mut lengths = Vec::with_capacity(indices.len());
    indices.iter().for_each(|x| {
        let min = view.offsets[*x as usize];
        // assumes `StringView` was constructed correctly
        let length = *unsafe { view.lengths.get_unchecked(*x as usize) };
        offsets.push(min);
        lengths.push(length);
    });

    StringView {
        values: view.values.clone(),
        offsets: Buffer::from_vec(offsets),
        lengths: Buffer::from_vec(lengths),
    }
}

pub fn take_array(array: &Utf8Array<i32>, indices: &[u64]) -> Utf8Array<i32> {
    let mut values = Vec::with_capacity(0);
    let mut offsets = Vec::with_capacity(indices.len() + 1);
    let mut length = 0;
    offsets.push(length);
    indices.iter().for_each(|x| {
        let min = array.offsets()[*x as usize];
        let max = array.offsets()[*x as usize + 1];
        length += max - min;
        offsets.push(length);
        values.extend_from_slice(&array.values()[min as usize..max as usize]);
    });

    // Soundness: slices of Utf8Array taken from offsets are by construction valid utf8
    unsafe {
        Utf8Array::from_data_unchecked(
            array.data_type().clone(),
            Buffer::from_vec(offsets),
            Buffer::from_vec(values),
            array.validity().cloned(),
        )
    }
}
