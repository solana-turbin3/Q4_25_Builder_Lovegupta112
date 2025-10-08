import bs58 from "bs58";
import promptSync from "prompt-sync";

const prompt = promptSync();

function base58ToWallet() {
  const base58 = prompt("Enter a base58 string: ");
  try {
    const wallet = bs58.decode(base58);
    console.log("Decoded wallet bytes:", wallet);
  } catch (err) {
    console.error("Invalid base58 input:", err);
  }
}


base58ToWallet();