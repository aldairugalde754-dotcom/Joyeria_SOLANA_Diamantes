import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { DiamondCert } from "../target/types/diamond_cert";
import {
  getAssociatedTokenAddressSync,
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
} from "@solana/spl-token";

async function ejecutarFlujoCertificacion() {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.DiamondCert as Program<DiamondCert>;

  const autoridadJoyeria = provider.wallet;
  const primerComprador = anchor.web3.Keypair.generate();
  const segundoComprador = anchor.web3.Keypair.generate();

  // Identificador unico para el certificado de esta sesion
  const identificadorSerie = "CERT-V1-" + Date.now().toString().slice(-6);

  // Derivacion de cuentas PDA (Program Derived Addresses)
  const [joyeriaPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("joyeria"), autoridadJoyeria.publicKey.toBuffer()],
    program.programId
  );

  const [joyeriaAuthPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("joyeria_auth"), joyeriaPda.toBuffer()],
    program.programId
  );

  const [certificadoPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("certificado"), Buffer.from(identificadorSerie)],
    program.programId
  );

  const [mintPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("mint"), Buffer.from(identificadorSerie)],
    program.programId
  );

  console.log("Configurando cuentas de prueba...");
  const txFondos = new anchor.web3.Transaction().add(
    anchor.web3.SystemProgram.transfer({
      fromPubkey: autoridadJoyeria.publicKey,
      toPubkey: primerComprador.publicKey,
      lamports: 0.05 * anchor.web3.LAMPORTS_PER_SOL,
    }),
    anchor.web3.SystemProgram.transfer({
      fromPubkey: autoridadJoyeria.publicKey,
      toPubkey: segundoComprador.publicKey,
      lamports: 0.05 * anchor.web3.LAMPORTS_PER_SOL,
    })
  );
  await provider.sendAndConfirm(txFondos);

  // 1. Registro de Entidad Emisora
  console.log("Ejecutando: registrar_joyeria");
  try {
    await program.methods
      .registrarJoyeria("Certificadora Diamantes Pro", "Suiza", "CH-CERT-8821")
      .accounts({
        autoridad: autoridadJoyeria.publicKey,
        joyeria: joyeriaPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
  } catch (e) {
    console.log(
      "Informacion: La entidad ya se encuentra registrada en la red."
    );
  }

  // 2. Emision de Certificado Digital (NFT)
  console.log(`Ejecutando: emitir_certificado (${identificadorSerie})`);
  const ataPrimerComprador = getAssociatedTokenAddressSync(
    mintPda,
    primerComprador.publicKey
  );

  await program.methods
    .emitirCertificado({
      numeroSerie: identificadorSerie,
      quilates: 175,
      corte: "Radiante",
      color: "G",
      claridad: "VVS1",
      precioUsd: new anchor.BN(9200),
    })
    .accounts({
      autoridad: autoridadJoyeria.publicKey,
      joyeria: joyeriaPda,
      joyeriaAuthPda: joyeriaAuthPda,
      certificado: certificadoPda,
      mint: mintPda,
      tokenAccountComprador: ataPrimerComprador,
      comprador: primerComprador.publicKey,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      systemProgram: anchor.web3.SystemProgram.programId,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
    })
    .rpc();

  // 3. Transferencia de Propiedad
  console.log("Ejecutando: transferir_certificado");
  const ataSegundoComprador = getAssociatedTokenAddressSync(
    mintPda,
    segundoComprador.publicKey
  );

  await program.methods
    .transferirCertificado(new anchor.BN(9800))
    .accounts({
      vendedor: primerComprador.publicKey,
      comprador: segundoComprador.publicKey,
      certificado: certificadoPda,
      mint: mintPda,
      tokenVendedor: ataPrimerComprador,
      tokenComprador: ataSegundoComprador,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .signers([primerComprador])
    .rpc();

  // 4. Reporte de Incidencia (Robo)
  console.log("Ejecutando: reportar_robo");
  await program.methods
    .reportarRobo("OFFICIAL-REP-5541")
    .accounts({
      reportante: segundoComprador.publicKey,
      certificado: certificadoPda,
    })
    .signers([segundoComprador])
    .rpc();

  // 5. Actualizacion de Tasacion
  console.log("Ejecutando: actualizar_tasacion");
  await program.methods
    .actualizarTasacion(new anchor.BN(10500), "Revision trimestral de mercado")
    .accounts({
      autoridad: autoridadJoyeria.publicKey,
      joyeria: joyeriaPda,
      certificado: certificadoPda,
    })
    .rpc();

  // Verificacion Final de Estado
  const estadoFinal = await program.account.certificado.fetch(certificadoPda);
  console.log("\n--- REPORTE FINAL DE CERTIFICADO ---");
  console.log("Serie:", estadoFinal.numeroSerie);
  console.log("Propietario Actual:", estadoFinal.propietario.toBase58());
  console.log("Valuacion:", estadoFinal.precioUsd.toString(), "USD");
  console.log("Estado:", JSON.stringify(estadoFinal.estado));
  console.log("Total Transferencias:", estadoFinal.numTransferencias);
  console.log("Registros en Historial:", estadoFinal.historial.length);
  console.log("------------------------------------\n");
}

ejecutarFlujoCertificacion().catch((error) => {
  console.error("Error en ejecucion:", error);
});
