import { DirectSecp256k1HdWallet } from '@cosmjs/proto-signing';
import { Slip10RawIndex } from '@cosmjs/crypto';

export const mnemonic =
  'salad system bind win endorse foster escape siren present rack resist hero tumble review ugly hat want whisper excite dirt execute area drum roast';
export const hidNodeEp = {
  rpc: 'https://rpc.prajna.hypersign.id',
  rest: 'https://api.prajna.hypersign.id',
  namespace: 'testnet',
};

export function makeCosmoshubPath(a) {
  return [
    Slip10RawIndex.hardened(44),
    Slip10RawIndex.hardened(118),
    Slip10RawIndex.hardened(0),
    Slip10RawIndex.normal(0),
    Slip10RawIndex.normal(a),
  ];
}

export const createWallet = async (mnemonic) => {
  let options;
  if (!mnemonic) {
    return await DirectSecp256k1HdWallet.generate(
      24,
      (options = {
        prefix: 'hid',
        hdPaths: [makeCosmoshubPath(0)],
      })
    );
  } else {
    return await DirectSecp256k1HdWallet.fromMnemonic(
      mnemonic,
      (options = {
        prefix: 'hid',
        hdPaths: [makeCosmoshubPath(0)],
      })
    );
  }
};

