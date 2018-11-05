//! Contains utils to print meshes.
use prettytable::{self, cell, row, Cell, Row, Table};
use data::GetMesh;
use std::io::{self, Write};
use std::collections::HashSet;

pub fn print_metadata<M: GetMesh>(mesh: M) -> io::Result<()> {
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
    GII: ReadEntity,
    W: Write,
{
    writeln!(w, "{}:", header)?;
    let mut table = Table::new();
    table.set_titles(row!["#", "name", "len", "attrs"]);
    let mut i = 0;
    for group in groups {
        // Collect all available attrs.
        let gd = group.metadata();
        let mut attr_names = HashSet::new();
        for i_item in 0..gd.len() {
            let item = group.item_at(i_item).expect("broken invariant");
            let n_attrs = item.attrs_len();
        }

        table.add_row(row![i, gd.name().get_original().0, gd.len(), ""]);
        i += 1;
    }
    if i > 0 {
        table.set_format(prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE.clone());
        table.print(&mut w);
    } else {
        writeln!(w, "There are no such groups.")?;
    }
    Ok(())
}
