use crate::tz::{TimeZone, TimeZoneNameIter};

mod tzs;

pub(crate) struct Database;

impl Database {
    pub(crate) fn new() -> Database {
        Database
    }

    pub(crate) fn reset(&self) {}

    pub(crate) fn get(&self, name: &str) -> Option<TimeZone> {
        // Check for the special `Etc/Unknown` value, which isn't in the
        // IANA time zone database.
        if name == "Etc/Unknown" {
            return Some(TimeZone::unknown());
        }
        let index = tzs::TZS
            .binary_search_by(|tz| {
                cmp_ignore_ascii_case(name, tz.iana_name().unwrap_or_default())
            })
            .ok()?;
        Some(unsafe { tzs::TZS[index].copy() })
    }

    pub(crate) fn available<'d>(&'d self) -> TimeZoneNameIter<'d> {
        TimeZoneNameIter::from_iter(
            tzs::TZS.iter().filter_map(|tz| tz.iana_name()),
        )
    }

    pub(crate) fn is_definitively_empty(&self) -> bool {
        false
    }
}

impl core::fmt::Debug for Database {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Bundled(available)")
    }
}

/// Like std's `eq_ignore_ascii_case`, but returns a full `Ordering`.
fn cmp_ignore_ascii_case(s1: &str, s2: &str) -> core::cmp::Ordering {
    let it1 = s1.as_bytes().iter().map(|&b| b.to_ascii_lowercase());
    let it2 = s2.as_bytes().iter().map(|&b| b.to_ascii_lowercase());
    it1.cmp(it2)
}

#[cfg(test)]
mod tests {
    use core::cmp::Ordering;

    use tzs::TZS;

    use super::*;

    /// This is a regression test where TZ names were sorted lexicographically
    /// but case sensitively, and this could subtly break binary search.
    #[test]
    fn sorted_ascii_case_insensitive() {
        for window in TZS.windows(2) {
            let name1 = window[0].iana_name().unwrap_or_default();
            let name2 = window[1].iana_name().unwrap_or_default();
            assert_eq!(
                Ordering::Less,
                cmp_ignore_ascii_case(name1, name2),
                "{name1} should be less than {name2}",
            );
        }
    }
}
