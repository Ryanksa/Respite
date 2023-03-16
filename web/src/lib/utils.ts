export const u8IntArrayToBase64 = (u8: Uint8Array) => {
  const base64String = btoa(
    // @ts-ignore
    String.fromCharCode.apply(null, u8)
  );
  return `data:image/png;base64,${base64String}`;
};
