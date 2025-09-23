export class BevyImageDecoder {
  constructor() {
    this._width = 0;
    this._height = 0;

    this.usingImageDecoder = BevyImageDecoder.supportsImageDecoder();
  }

  static supportsImageDecoder() {
    return 'ImageDecoder' in window;
  }

  width() {
    return this._width;
  }

  height() {
    return this._height;
  }

  async decode(data, mimeType) {
    if (this.usingImageDecoder) {
      this.data = await decodeImageWithImageDecoder(data, mimeType);
      this._width = this.data.codedWidth;
      this._height = this.data.codedHeight;
    } else {
      const decoded = await decodeImageWithCanvas(data, mimeType);
      this.data = decoded.data;
      this._width = decoded.width;
      this._height = decoded.height;
    }
  }

  copy(buffer) {
    if (this.usingImageDecoder) {
      return this.data.copyTo(buffer, { format: 'RGBA' });
    } else {
      buffer.set(this.data);
      return Promise.resolve();
    }
  }
}

/**
 * @param {Uint8Array} buffer
 * @returns {Promise<{ data: Uint8ClampedArray width: number, height: number }>}
 */
function decodeImageWithCanvas(buffer, mimeType) {
  return new Promise((resolve, reject) => {
    const blob = new Blob([buffer], { type: mimeType });
    const url = URL.createObjectURL(blob);
    const img = new Image();
    img.src = url;
    img.onload = (e) => {
      URL.revokeObjectURL(url);

      const canvas = document.createElement('canvas');
      canvas.width = img.width;
      canvas.height = img.height;

      const ctx = canvas.getContext('2d');
      ctx.drawImage(img, 0, 0);

      const imageData = ctx.getImageData(0, 0, img.width, img.height);
      resolve({
        data: imageData.data,
        width: img.width,
        height: img.height,
      });
    };
    img.onerror = reject;
  });
}

/**
 * @param {Uint8Array} buffer
 * @returns {Promise<VideoFrame>}
 */
async function decodeImageWithImageDecoder(buffer, mimeType) {
  const decoder = new ImageDecoder({ type: mimeType, data: buffer });
  const decoded = await decoder.decode();
  return decoded.image;
}
