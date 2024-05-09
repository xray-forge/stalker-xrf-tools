export function blobToImage(blob: Blob): Promise<HTMLImageElement> {
  return new Promise((resolve): void => {
    const url: string = URL.createObjectURL(blob);
    const img: HTMLImageElement = new Image();

    img.onload = () => {
      // URL.revokeObjectURL(url);
      resolve(img);
    };

    img.src = url;
  });
}
