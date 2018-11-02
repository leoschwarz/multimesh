//! Contains utils to print meshes.
use prettytable::{self, cell, row, Cell, Row, Table};
use ser::{SerializableGroup, SerializableMesh};
use std::io::{self, Write};

pub fn print_metadata<M: SerializableMesh>(mesh: M) -> io::Result<()> {
    write_metadata(mesh, &mut ::std::io::stdout())
}

pub fn write_metadata<M: SerializableMesh, W: Write>(mesh: M, mut w: W) -> io::Result<()> {
    writeln!(w, "General metadata")?;
    writeln!(w, "----------------")?;
    writeln!(w, "Mesh dimension: {}", mesh.metadata().dimension())?;
    writeln!(w, "")?;

    write_metadata_groups("Node groups", mesh.node_groups(), &mut w);
    write_metadata_groups("Element groups", mesh.element_groups(), &mut w);
    write_metadata_groups("Vector groups", mesh.vector_groups(), &mut w);
    write_metadata_groups("Other groups", mesh.other_groups(), &mut w);

    Ok(())
}

fn write_metadata_groups<G, GI, GII, W>(header: &str, groups: G, mut w: W) -> io::Result<()>
where
    G: Iterator<Item = GI>,
    GI: SerializableGroup<Item = GII>,
    W: Write,
{
    writeln!(w, "{}:", header)?;
    let mut table = Table::new();
    table.add_row(row!["#", "name", "len"]);
    let mut i = 0;
    for group in groups {
        let gd = group.metadata();
        table.add_row(row![i, gd.name().get_original().0, gd.len()]);
        i += 1;
    }
    table.set_format(prettytable::format::consts::FORMAT_BOX_CHARS.clone());
    table.print(&mut w);
    Ok(())
}
