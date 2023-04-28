export const toImageUrl = (u8: Uint8Array) => {
  try {
    return URL.createObjectURL(
      new Blob([u8], {
        type: "image/*",
      })
    );
  } catch {
    return "";
  }
};
