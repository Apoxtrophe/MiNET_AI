use crate::Minet;
use base64::{engine::general_purpose, Engine as _};
use std::{error::Error, io::{Cursor, Read, Write}};


impl Minet {
    pub fn encode(&self) -> String {
    let mut bytes = Vec::new();

    // Serialize the number of genes as u16
    let num_genes = self.genes.len() as u16;
    bytes.extend_from_slice(&num_genes.to_le_bytes());
    
    for (gene_val, connections) in &self.genes {
        // Serialize the f32 gene value
        bytes.extend_from_slice(&gene_val.to_le_bytes());

        // Serialize the number of connections as u8
        let num_connections = connections.len() as u8;
        bytes.push(num_connections);

        for (index, conn_val) in connections {
            // Serialize the index as u8
            bytes.push(*index as u8);

            // Serialize the f32 connection value
            bytes.extend_from_slice(&conn_val.to_le_bytes());
            }
        }
    // Encode the byte vector to a base64 string
    general_purpose::STANDARD_NO_PAD.encode(&bytes)
    }
    
    pub fn import_base64(
        &mut self,
        encoded: &str,
    ) {
    // Decode the base64 string to bytes
    let bytes = general_purpose::STANDARD_NO_PAD.decode(encoded).unwrap();
    let mut cursor = Cursor::new(bytes);

    // Deserialize the number of genes
    let mut num_genes_bytes = [0u8; 2];
    cursor.read_exact(&mut num_genes_bytes).unwrap();
    let num_genes = u16::from_le_bytes(num_genes_bytes) as usize;

    let mut genes = Vec::new();

    for _ in 0..num_genes {
        // Deserialize the f32 gene value
        let mut gene_val_bytes = [0u8; 4];
        cursor.read_exact(&mut gene_val_bytes).unwrap();
        let gene_val = f32::from_le_bytes(gene_val_bytes);

        // Deserialize the number of connections
        let mut num_connections_bytes = [0u8; 1];
        cursor.read_exact(&mut num_connections_bytes).unwrap();
        let num_connections = num_connections_bytes[0] as usize;

        let mut connections = Vec::new();
        for _ in 0..num_connections {
            // Deserialize the index as u8
            let mut index_byte = [0u8; 1];
            cursor.read_exact(&mut index_byte).unwrap();
            let index = index_byte[0] as usize;

            // Deserialize the f32 connection value
            let mut conn_val_bytes = [0u8; 4];
            cursor.read_exact(&mut conn_val_bytes).unwrap();
            let conn_val = f32::from_le_bytes(conn_val_bytes);

            connections.push((index, conn_val));
            }

            genes.push((gene_val, connections));
        }
        self.genes = genes;
    }
}
