use super::*;


impl Debug for CycleElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("CycleElement(")?;
        fmt_numbers(f, self)?;
        f.write_str(")")
    }
}

impl Display for CycleElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Cycles[{{")?;
        fmt_numbers(f, self)?;
        f.write_str("}}]")
    }
}

impl Display for CycleNotation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Cycles[{")?;
        for (j, cycle) in self.cycles.iter().enumerate() {
            f.write_str("{")?;
            fmt_numbers(f, cycle)?;
            if j == self.cycles.len() - 1 {
                f.write_str("}")?;
            } else {
                f.write_str("}, ")?;
            }
        }
        f.write_str("}]")
    }
}

fn fmt_numbers(f: &mut Formatter<'_>, cycle: &CycleElement) -> std::fmt::Result {
    for (i, index) in cycle.chain.iter().enumerate() {
        if i == 0 {
            write!(f, "{}", index + 1)?;
        } else {
            write!(f, ", {}", index + 1)?;
        }
    }
    Ok(())
}