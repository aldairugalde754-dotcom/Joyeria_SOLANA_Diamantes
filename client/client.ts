import * as anchor from "@coral-xyz/anchor";

async function ejecutarJoyeria() {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.JoyeriaBlockchain;
  const owner = provider.wallet;

  const [inventarioPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("joyeria"), owner.publicKey.toBuffer()],
    program.programId
  );

  console.log("Iniciando operaciones de joyeria...");

  // 1. Inicializar Inventario
  try {
    await program.methods
      .inicializarInventario("Joyeria de Lujo Real")
      .accounts({
        inventario: inventarioPda,
        owner: owner.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
    console.log("Inventario creado con exito.");
  } catch (e) {
    console.log("El inventario ya existe, continuando...");
  }

  // 2. Registrar Diamante
  // Tip: Cambia este numero de serie en cada 'run' para ver como crece el stock
  const nSerie = "D-BR-77889911";
  const quilates = 250;

  await program.methods
    .registrarDiamante(nSerie, quilates)
    .accounts({
      inventario: inventarioPda,
      owner: owner.publicKey,
    })
    .rpc();
  console.log("Diamante " + nSerie + " registrado.");

  // 3. Ver Inventario
  const cuenta = await program.account.inventario.fetch(inventarioPda);
  console.log("Diamantes en stock: " + cuenta.diamantes.length);

  // 4. Alternar Autenticacion
  await program.methods
    .alternarAutenticacion(nSerie)
    .accounts({
      inventario: inventarioPda,
      owner: owner.publicKey,
    })
    .rpc();
  console.log("Estado de autenticacion actualizado.");

  // 5. Transferir Diamante (Comentado para mantener los datos)
  /*
  await program.methods
    .transferirDiamante(nSerie)
    .accounts({
      inventario: inventarioPda,
      owner: owner.publicKey,
    })
    .rpc();
  console.log("Diamante transferido y removido.");
  */
}

// Ejecucion del script
ejecutarJoyeria().catch((err) => {
  console.error("Error en la ejecucion:", err);
});
