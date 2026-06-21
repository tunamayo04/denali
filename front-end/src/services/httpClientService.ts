import axios, { type AxiosInstance, type AxiosRequestConfig, type AxiosResponse } from 'axios'
import { isFilter, serializeFilter } from '@/models/shared.ts'

// Walks a flat params object and turns any Filter<T> values into the
// `op:value` string the backend's Deserialize impl expects, leaving
// everything else untouched.
function serializeParams(params: unknown): unknown {
  if (params === null || typeof params !== 'object') return params
  const result: Record<string, unknown> = {}
  for (const [key, value] of Object.entries(params as Record<string, unknown>)) {
    if (value === undefined) continue
    result[key] = isFilter(value) ? serializeFilter(value) : value
  }
  return result
}

export class HttpClientService {
  private axiosClient: AxiosInstance
  constructor() {
    this.axiosClient = axios.create({
      baseURL: 'http://localhost:3000/',
      headers: {
        'Content-Type': 'application/json',
      },
    })
  }
  public async get<T>(
    url: string,
    params?: unknown,
    config?: AxiosRequestConfig,
  ): Promise<AxiosResponse<T>> {
    return this.axiosClient.get<T>(url, {
      ...config,
      params: serializeParams(params),
    })
  }
  public async post<T>(
    url: string,
    data?: unknown,
    config?: AxiosRequestConfig,
  ): Promise<AxiosResponse<T>> {
    return this.axiosClient.post<T>(url, data, config)
  }
  public async put<T>(
    url: string,
    data?: unknown,
    config?: AxiosRequestConfig,
  ): Promise<AxiosResponse<T>> {
    return this.axiosClient.put<T>(url, data, config)
  }
  public async delete<T>(
    url: string,
    params?: unknown,
    config?: AxiosRequestConfig,
  ): Promise<AxiosResponse<T>> {
    return this.axiosClient.delete<T>(url, {
      ...config,
      params: serializeParams(params),
    })
  }
}
