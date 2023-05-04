import { Component, OnInit, ViewChild, TemplateRef } from '@angular/core';
import { MatDialog } from '@angular/material/dialog';
import { HttpClient } from '@angular/common/http';
import { Router } from '@angular/router';

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
  @ViewChild('successModal', { static: false }) successModal!: TemplateRef<any>;
  
  withdrawalRequest: WithdrawalRequest = {
    account_id: '',
    amount: 0,
  };
  accounts: Account[] = [];
  selectedAccount: Account | null = null;

  constructor(private http: HttpClient, private router: Router, public dialog: MatDialog) { }

  async ngOnInit(): Promise<void> {
    await this.fetchAccounts();
  }

  private async fetchAccounts(): Promise<void> {
    const apiUrl = `http://localhost:5000/api/account/accounts`; // Replace with your actual API URL

    try {
      const response = await this.http.get<{ accounts: any[] }>(apiUrl).toPromise(); 

      if (!response)
        return;

      this.accounts = response.accounts;
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
    const apiUrl = 'http://localhost:5000/api/bank/withdraw'; // Replace with your actual API URL

    this.http.post(apiUrl, this.withdrawalRequest).subscribe(
      (response) => {
        console.log('Withdrawal successful:', response);
        this.openSuccessModal();
      },
      (error) => console.error('Error during withdrawal:', error)
    );
  }

  openSuccessModal(): void {
    this.dialog.open(this.successModal);
  }

  redirectToAccount(): void {
    this.router.navigate(['/account']);
  }
}
