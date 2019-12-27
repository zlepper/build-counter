import { environment } from '../../../environments/environment';

export function getLoginUrl(): string {
  const api = environment.apiServerUrl;

  const returnUrl = encodeURIComponent(location.href);

  return `${api}start-gh-login?return_url=${returnUrl}`;
}
