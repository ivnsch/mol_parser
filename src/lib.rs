mod element;
mod parser;

#[cfg(test)]
mod tests {
    use element::Element;
    use parser::parse_mol2;

    use super::*;

    #[tokio::test]
    async fn it_works() {
        let file = "assets/benzene.mol2";
        let mol_res = parse_mol2(&file, 1).await;
        assert!(mol_res.is_ok());

        let mol = mol_res.unwrap();
        assert_eq!(12, mol.atoms.len());
        assert_eq!(12, mol.bonds.len());

        assert_eq!(Element::C, mol.atoms[0].element);
        assert_eq!(Element::C, mol.atoms[1].element);
        assert_eq!(Element::C, mol.atoms[2].element);
        assert_eq!(Element::C, mol.atoms[3].element);
        assert_eq!(Element::C, mol.atoms[4].element);
        assert_eq!(Element::C, mol.atoms[5].element);
        assert_eq!(Element::H, mol.atoms[6].element);
        assert_eq!(Element::H, mol.atoms[7].element);
        assert_eq!(Element::H, mol.atoms[8].element);
        assert_eq!(Element::H, mol.atoms[9].element);
        assert_eq!(Element::H, mol.atoms[10].element);
        assert_eq!(Element::H, mol.atoms[11].element);
    }
}
