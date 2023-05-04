import { Component, OnInit, ViewChild, TemplateRef } from '@angular/core';
import { MatDialog } from '@angular/material/dialog';
import { HttpClient } from '@angular/common/http';
import { Router } from '@angular/router';

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
  @ViewChild('successModal', { static: false }) successModal!: TemplateRef<any>;

  depositRequest: DepositRequest = {
    from_account_id: '',
    to_account_id: '',
    amount: 0,
    is_bank_agent: false
  };
  accounts: any[] = [];
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
      this.depositRequest.from_account_id = this.selectedAccount?.account_id || '';
    } catch (error) {
      console.error('Error fetching accounts:', error);
    }
  }

  onAccountChange(event: Event): void {
    this.depositRequest.from_account_id = this.selectedAccount?.account_id || '';
  }

  onSubmit(): void {
    const apiUrl = 'http://localhost:5000/api/bank/deposit'; // Replace with your actual API URL
    this.http.post(apiUrl, this.depositRequest).subscribe(
      (response) => {
        console.log('Deposit successful:', response);
        this.openSuccessModal();
      },
      (error) => console.error('Error during deposit:', error)
    );
  }

  openSuccessModal(): void {
    this.dialog.open(this.successModal);
  }

  redirectToAccount(): void {
    this.router.navigate(['/account']);
  }
}
