import { Component, OnInit } from '@angular/core';
import { HttpClient } from '@angular/common/http';

interface DepositRequest {
  from_account_id: string;
  to_account_id: string;
  amount: number;
  is_bank_agent: boolean;
}

interface Account {
  account_name: string;
  account_id: string;
}

@Component({
  selector: 'app-deposit',
  templateUrl: './deposit.component.html',
  styleUrls: ['./deposit.component.scss']
})
export class DepositComponent implements OnInit {
  depositRequest: DepositRequest = {
    from_account_id: '',
    to_account_id: '',
    amount: 0,
    is_bank_agent: false
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
      this.depositRequest.from_account_id = this.selectedAccount.account_id;
    } catch (error) {
      console.error('Error fetching accounts:', error);
    }
  }

  onAccountChange(event: Event): void {
    this.depositRequest.from_account_id = this.selectedAccount?.account_id || '';
  }

  onSubmit(): void {
    const apiUrl = 'http://localhost:8000/deposit'; // Replace with your actual API URL
    this.http.post(apiUrl, this.depositRequest).subscribe(
      (response) => console.log('Deposit successful:', response),
      (error) => console.error('Error during deposit:', error)
    );
  }
}
