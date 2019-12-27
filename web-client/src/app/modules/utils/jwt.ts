// Stolen from https://stackoverflow.com/a/38552302/3950006, with slightly modifications for coding style
export function parseJwt<T>(token: string): T {
  const base64Url = token.split('.')[1];
  const base64 = base64Url.replace(/-/g, '+').replace(/_/g, '/');
  const jsonPayload = decodeURIComponent(
    atob(base64)
      .split('')
      .map(c => '%' + ('00' + c.charCodeAt(0).toString(16)).slice(-2))
      .join(''),
  );

  return JSON.parse(jsonPayload) as T;
}
