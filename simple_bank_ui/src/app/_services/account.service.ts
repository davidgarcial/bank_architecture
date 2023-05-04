import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';

const API_URL = 'http://localhost:5000/api/account/';

@Injectable({
  providedIn: 'root',
})
export class AccountService {
  constructor(private http: HttpClient) {}

  createAccount(accountType: string, accountName: string): Observable<any> {
    return this.http.post(API_URL + 'create', {
      account_type: accountType,
      account_name: accountName,
    });
  }

  getAccount(accountId: string): Observable<any> {
    return this.http.get(API_URL + accountId);
  }

  getAccounts(): Observable<any> {
    return this.http.get(API_URL + 'accounts');
  }

  updateAccount(accountId: string, accountName: string, newBalance: number): Observable<any> {
    return this.http.put(API_URL + 'update', {
      account_id: accountId,
      account_name: accountName,
      balance: newBalance,
    });
  }

  getTransactionHistory(accountId: string): Observable<any> {
    return this.http.get('http://localhost:5000/api/history/transactions/' + accountId);
  }
}
