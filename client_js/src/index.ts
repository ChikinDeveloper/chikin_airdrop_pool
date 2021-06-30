import { Connection, PublicKey, TransactionInstruction } from "@solana/web3.js";
import * as BufferLayout from "buffer-layout";

export const AirdropPoolLayout = BufferLayout.struct([

]);

export class AirdropPool {
    static async getMinBalanceRentForExempt(
        connection: Connection,
    ): Promise<number> {
        return await connection.getMinimumBalanceForRentExemption(AirdropPoolLayout.span);
    }

    static async create(
        connection: Connection,
        creator: PublicKey,
        program: PublicKey,
        rentSysvar: PublicKey,
        systemProgram: PublicKey,
        tokenProgram: PublicKey,
        tokenMint: PublicKey,
        programAccount: PublicKey,
        programTokenAccount: PublicKey,
    ): Promise<AirdropPool> {
        const instruction = this.createInitializeInstruction(
            creator,
            program,
            rentSysvar,
            systemProgram,
            tokenProgram,
            tokenMint,
            programAccount,
            programTokenAccount,
        );

        connection.send
    }

    async claim(): Promise<void> {

    }

    static createInitializeInstruction(
        creator: PublicKey,
        program: PublicKey,
        rentSysvar: PublicKey,
        systemProgram: PublicKey,
        tokenProgram: PublicKey,
        tokenMint: PublicKey,
        programAccount: PublicKey,
        programTokenAccount: PublicKey,
    ): TransactionInstruction {
        const keys = [
            { pubkey: creator, isSigner: false, isWritable: false },
            { pubkey: program, isSigner: false, isWritable: false },
            { pubkey: rentSysvar, isSigner: false, isWritable: false },
            { pubkey: systemProgram, isSigner: false, isWritable: false },
            { pubkey: tokenProgram, isSigner: false, isWritable: false },
            { pubkey: tokenMint, isSigner: false, isWritable: false },
            { pubkey: programAccount, isSigner: false, isWritable: false },
            { pubkey: programTokenAccount, isSigner: false, isWritable: false },
        ];

        const dataLayout = BufferLayout.struct([
            BufferLayout.u8('type'),
        ]);
        let data = Buffer.alloc(1024);
        {
            const encodeLength = dataLayout.encode(
                {
                    type: 0,
                },
                data,
            );
            data = data.slice(0, encodeLength);
        }

        return new TransactionInstruction({
            keys,
            programId: program,
            data,
        });
    }
}