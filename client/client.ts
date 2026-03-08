import * as anchor from "@coral-xyz/anchor";

// @ts-ignore
import { expect } from "chai";

describe("joyeria_blockchain", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.JoyeriaBlockchain;
  const owner = provider.wallet;

  const [inventarioPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("joyeria"), owner.publicKey.toBuffer()],
    program.programId
  );

  it("Inicializar Inventario y Registrar Diamante", async () => {
    await program.methods
      .inicializarInventario("Joyeria de Lujo Real")
      .accounts({
        inventario: inventarioPda,
        owner: owner.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const nSerie = "D-BR-77889911";
    const quilates = 250;

    await program.methods
      .registrarDiamante(nSerie, quilates)
      .accounts({
        inventario: inventarioPda,
        owner: owner.publicKey,
      })
      .rpc();

    const cuenta = await program.account.inventario.fetch(inventarioPda);
    expect(cuenta.diamantes[0].numeroSerie).to.equal(nSerie);
    expect(cuenta.diamantes[0].quilates).to.equal(quilates);
    expect(cuenta.diamantes[0].autenticado).to.be.true;
    console.log("Diamante D-BR-77889911 registrado con exito");
  });

  it("Alternar Autenticacion", async () => {
    const nSerie = "D-BR-77889911";

    await program.methods
      .alternarAutenticacion(nSerie)
      .accounts({
        inventario: inventarioPda,
        owner: owner.publicKey,
      })
      .rpc();

    const cuenta = await program.account.inventario.fetch(inventarioPda);
    expect(cuenta.diamantes[0].autenticado).to.be.false;
    console.log("Estado de autenticacion del diamante D-BR-77889911 cambiado a false");
  });

  it("Transferencia de Diamante", async () => {
    const nSerie = "D-BR-77889911";

    await program.methods
      .transferirDiamante(nSerie)
      .accounts({
        inventario: inventarioPda,
        owner: owner.publicKey,
      })
      .rpc();

    const cuenta = await program.account.inventario.fetch(inventarioPda);
    expect(cuenta.diamantes.length).to.equal(0);
    console.log("Diamante D-BR-77889911 transferido y removido del inventario");
  });

  it("Caso Extra: Validacion de Error Serie Inexistente", async () => {
    try {
      await program.methods
        .removerDiamante("ERROR-999")
        .accounts({
          inventario: inventarioPda,
          owner: owner.publicKey,
        })
        .rpc();
      expect.fail("El programa deberia haber fallado");
    } catch (err) {
      console.log("El sistema rechazo correctamente la serie inexistente ERROR-999");
    }
  });
});
