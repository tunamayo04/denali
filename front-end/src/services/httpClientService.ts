import axios, { type AxiosInstance, type AxiosRequestConfig, type AxiosResponse } from 'axios'

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
      params,
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
      params,
    })
  }
}
