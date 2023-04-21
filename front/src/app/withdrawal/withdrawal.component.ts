import { Component, OnInit } from '@angular/core';
import { HttpClient } from '@angular/common/http';

interface WithdrawalRequest {
  account_id: string;
  amount: number;
}

interface Account {
  account_name: string;
  account_id: string;
}

@Component({
  selector: 'app-withdrawal',
  templateUrl: './withdrawal.component.html',
  styleUrls: ['./withdrawal.component.scss']
})
export class WithdrawalComponent implements OnInit {
  withdrawalRequest: WithdrawalRequest = {
    account_id: '',
    amount: 0,
  };
  accounts: Account[] = [];
  selectedAccount: Account | null = null;

  constructor(private http: HttpClient) { }

  async ngOnInit(): Promise<void> {
    await this.fetchAccounts();
  }

  private async fetchAccounts(): Promise<void> {
    const user_id = 'example_user_id'; // Replace with the actual user ID
    const apiUrl = `http://localhost:8000/accounts/${user_id}`; // Replace with your actual API URL

    try {
      const response = await this.http.get<Account[]>(apiUrl).toPromise();

      if (!response)
        return;
        
      this.accounts = response;
      this.selectedAccount = this.accounts[0];
      this.withdrawalRequest.account_id = this.selectedAccount.account_id;
    } catch (error) {
      console.error('Error fetching accounts:', error);
    }
  }

  onAccountChange(event: Event): void {
    this.withdrawalRequest.account_id = this.selectedAccount?.account_id || '';
  }

  onSubmit(): void {
    const apiUrl = 'http://localhost:8000/withdraw'; // Replace with your actual API URL

    this.http.post(apiUrl, this.withdrawalRequest).subscribe(
      (response) => console.log('Withdrawal successful:', response),
      (error) => console.error('Error during withdrawal:', error)
    );
  }
}
