import { Component, OnInit, Inject } from '@angular/core';
import { MatDialogRef, MAT_DIALOG_DATA } from '@angular/material/dialog';
import { AccountService } from '../_services/account.service';

@Component({
  selector: 'app-history',
  templateUrl: './history.component.html',
  styleUrls: ['./history.component.scss'],
})
export class HistoryComponent implements OnInit {
  accountId: string = '';
  transactions: any[] = [];
  displayedColumns: string[] = [
    'transaction_id',
    'account_id',
    'transaction_type',
    'amount',
    'timestamp'
  ];
  

  constructor(
    private accountService: AccountService,
    public dialogRef: MatDialogRef<HistoryComponent>,
    @Inject(MAT_DIALOG_DATA) public data: any
  ) {}

  ngOnInit(): void {
    this.getTransactionHistory();
  }

  getTransactionHistory() {
    this.accountService.getTransactionHistory(this.data.account).subscribe(
      (response) => {
        this.transactions = response.data.transactions;
      },
      (error) => {
        console.error('Error fetching transaction history', error);
      }
    );
  }
}
